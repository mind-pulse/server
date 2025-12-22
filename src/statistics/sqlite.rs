use std::collections::HashMap;

use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use time::macros::offset;
use tokio::sync::OnceCell;

use crate::{
    error::{MindPulseError, MindPulseResult},
    scale::{get_scale_id_by_path, PATHS},
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
pub(super) struct ScaleStatistics<'a> {
    name: &'a str,
    count: u64,
}

/// 客户端类型枚举
#[derive(sqlx::Type, Default, Debug, Clone, Copy)]
#[repr(u8)]
enum ClientType {
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

/// 创建统计数据表
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

/// 查询单个量表的统计数据
pub(super) async fn query_scale_statistics(
    scale_path: &str,
) -> MindPulseResult<ScaleStatistics<'_>> {
    trace!(message = "Querying scale statistics", scale = scale_path);

    let pool = get_database_pool().await;

    let scale_index = get_scale_id_by_path(scale_path)?;
    debug!(
        message = "Resolved scale index",
        index = scale_index,
        scale = scale_path
    );

    debug!(message = "Fetching test count", scale = scale_path);
    let (count,): (u64,) = sqlx::query_as(
        "SELECT COUNT(*) as times FROM statistics_ip WHERE scale_index = $1",
    )
    .bind(scale_index as i32)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!(message = "Failed to query scale statistics", scale = scale_path, error = ?e);
        e
    })?;

    info!(
        message = "Scale statistics retrieved",
        scale = scale_path,
        count = count
    );

    Ok(ScaleStatistics {
        name: scale_path,
        count,
    })
}

/// 查询所有量表的统计数据
pub(super) async fn query_all_statistics<'a>() -> MindPulseResult<HashMap<u16, ScaleStatistics<'a>>>
{
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
    let mut statistics_map: HashMap<u16, ScaleStatistics<'_>> = PATHS
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

/// 插入完成的测试记录
pub(super) async fn insert_completed_test(
    id: u16,
    client_type: u8,
    ip_address: &str,
) -> MindPulseResult<u64> {
    trace!(
        message = "Converting client type",
        client_type = client_type
    );
    let client_type: ClientType = client_type.try_into().map_err(|e| {
        error!(message = "Failed to convert client type", error = ?e);
        e
    })?;
    debug!(message = "Client type converted successfully", client_type = ?client_type);

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
