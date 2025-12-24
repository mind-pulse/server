use std::collections::HashMap;

use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use time::macros::offset;
use tokio::sync::OnceCell;

use crate::{
    error::{MindPulseError, MindPulseResult},
    scale::{get_scale_name_by_id, LIST},
};

type SqlitePool = Pool<Sqlite>;

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

/// 获取全局 SQLite 数据库连接池
async fn get_database_pool() -> &'static SqlitePool {
    SQLITE_POOL
        .get_or_init(|| async {
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect("./confidant.sqlite?mode=rwc")
                .await
                .map_err(|e| {
                    error!(message = "Failed to create database connection pool", error = ?e);
                    e
                })
                .unwrap()
        })
        .await
}

/// 量表统计数据结构
#[derive(Debug, Serialize)]
pub struct ScaleStatistics<'a> {
    name: &'a str,
    count: u64,
}

/// 客户端类型枚举
#[derive(sqlx::Type, Default, Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum ClientType {
    Wechat = 1,
    #[default]
    MobileBrowser,
}

impl TryFrom<u8> for ClientType {
    type Error = MindPulseError;

    fn try_from(value: u8) -> MindPulseResult<Self> {
        match value {
            1 => Ok(ClientType::Wechat),
            2 => Ok(ClientType::MobileBrowser),
            invalid => {
                error!(message = "Invalid client type", value = invalid);
                Err(MindPulseError::InvalidClientType(value))
            }
        }
    }
}

/// Ensure the `statistics_ip` table exists in the configured SQLite database.
///
/// The table created (if missing) has columns: `id` (primary key), `scale_index`,
/// `ip`, `client_type`, and `finished_time`. Returns `Ok(())` on success or an
/// error if the database operation fails.
///
/// # Examples
///
/// ```
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// // Ensure a Tokio runtime is available when running this example.
/// tokio::runtime::Runtime::new()?.block_on(async {
///     create_statistics_table().await?;
///     Ok::<(), Box<dyn std::error::Error>>(())
/// })?;
/// # Ok(()) }
/// ```
pub async fn create_statistics_table() -> MindPulseResult<()> {
    trace!(message = "Creating statistics table");

    let pool = get_database_pool().await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS statistics_ip (
                id            INTEGER PRIMARY KEY,
                scale_index   INTEGER NOT NULL,
                ip            VARCHAR(15) NOT NULL DEFAULT '',
                client_type   INTEGER NOT NULL,
                finished_time DATETIME NOT NULL
            )",
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!(message = "Failed to create statistics table", error = ?e);
        e
    })?;

    info!(
        message = "Statistics table created or already exists",
        table = "statistics_ip"
    );

    Ok(())
}

/// Retrieve statistics for a single scale by its ID.
///
/// Returns `Ok(ScaleStatistics)` containing the scale's resolved name and the total number of recorded completions for that scale. Returns `Err` if the ID cannot be resolved or if the database query fails.
///
/// # Examples
///
/// ```
/// # tokio_test::block_on(async {
/// let stats = query_scale_statistics(1).await.unwrap();
/// assert!(!stats.name.is_empty());
/// assert!(stats.count >= 0);
/// # });
/// ```
pub async fn query_scale_statistics(id: u16) -> MindPulseResult<ScaleStatistics<'static>> {
    trace!(message = "Querying scale statistics", id);

    let pool = get_database_pool().await;

    // 验证 ID
    let name = get_scale_name_by_id(id)?;
    debug!(
        message = "Successfully resolved scale index",
        name = name,
        id = id
    );

    debug!(message = "Fetching test count", id);
    let (count,): (u64,) =
        sqlx::query_as("SELECT COUNT(*) as times FROM statistics_ip WHERE scale_index = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                error!(message = "Failed to query scale statistics", id, error = ?e);
                e
            })?;

    info!(message = "Scale statistics retrieved", id, count);

    Ok(ScaleStatistics { name, count })
}

/// Retrieves statistics for every scale.
///
/// The returned map contains one entry per known scale id; each value is a `ScaleStatistics` with the scale's name and the total number of completed tests recorded for that scale. Scales with no recorded entries will appear with a `count` of 0.
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use your_crate::{query_all_statistics, ScaleStatistics};
/// # tokio_test::block_on(async {
/// let stats: HashMap<u16, ScaleStatistics<'_>> = query_all_statistics().await.unwrap();
/// // `stats` maps scale id -> ScaleStatistics { name, count }
/// assert!(stats.len() >= 0);
/// # });
/// ```
pub async fn query_all_statistics<'a>() -> MindPulseResult<HashMap<u16, ScaleStatistics<'a>>> {
    let pool = get_database_pool().await;

    // scale_index 为旧版量表的索引，现在已改为 id
    trace!(message = "Querying all scale statistics");
    let rows: Vec<(u16, u64)> = sqlx::query_as(
        "SELECT scale_index, COUNT(*) as count
         FROM statistics_ip
         GROUP BY scale_index",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(message = "Failed to query all statistics", error = ?e);
        e
    })?;

    // 构建完整的统计映射，包含未有任何记录的量表：HashMap<id, ScaleStatistics<'_>>
    let mut statistics_map: HashMap<u16, ScaleStatistics<'_>> = LIST
        .iter()
        .map(|p| {
            (
                p.id(),
                ScaleStatistics {
                    name: p.name(),
                    count: 0,
                },
            )
        })
        .collect();

    // 更新实际统计数据
    for (id, count) in rows.iter() {
        if let Some(stats) = statistics_map.get_mut(id) {
            stats.count = *count;
        }
    }

    info!(message = "All scale statistics retrieved", statistics = ?statistics_map);

    Ok(statistics_map)
}

/// Insert a completed test record for a scale into the statistics table.
///
/// The function records the scale id, client type, IP address and the current
/// timestamp (UTC+8) into the `statistics_ip` table. It returns the number of
/// rows affected by the insert.
///
/// # Parameters
///
/// - `id`: The scale identifier to store (stored in the column still named `scale_index` for compatibility).
/// - `client_type`: The client type that completed the test.
/// - `ip_address`: The caller's IP address.
///
/// # Returns
///
/// The number of rows affected by the insert.
///
/// # Examples
///
/// ```
/// // Run the async call on a runtime to insert a test record.
/// let rt = tokio::runtime::Runtime::new().unwrap();
/// let rows = rt.block_on(async {
///     insert_completed_test(1, crate::ClientType::MobileBrowser, "127.0.0.1").await
/// }).unwrap();
/// assert!(rows == 1 || rows == 0);
/// ```
pub async fn insert_completed_test(
    id: u16,
    client_type: ClientType,
    ip_address: &str,
) -> MindPulseResult<u64> {
    trace!(
        message = "Converting client type",
        client_type = ?client_type
    );

    trace!(message = "Getting current timestamp");
    let timestamp = time::OffsetDateTime::now_utc().to_offset(offset!(+8));
    debug!(message = "Current timestamp obtained", timestamp = ?timestamp);

    let pool = get_database_pool().await;

    trace!(
        message = "Inserting test record",
        scale_index = id,
        ip_address = ip_address,
        client_type = ?client_type
    );

    // 为了兼容旧表，id 列名暂时仍使用 scale_index
    let result = sqlx::query(
        "INSERT INTO statistics_ip (scale_index, ip, finished_time, client_type) VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(ip_address)
    .bind(timestamp)
    .bind(client_type)
    .execute(pool)
    .await
    .map_err(|e| {
        error!(
            message = "Failed to insert test record",
            error = ?e,
            scale_index = id,
            client_type = ?client_type
        );
        e
    })?;

    info!(
        message = "Test record inserted successfully",
        scale_index = id,
        ip_address = ip_address,
        client_type = ?client_type
    );

    Ok(result.rows_affected())
}