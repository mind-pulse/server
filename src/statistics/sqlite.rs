use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use time::macros::offset;
use tokio::sync::OnceCell;

use crate::{
    error::{MindPulseError, MindPulseResult},
    scale::{get_scale_index_by_path, PATHS},
};

type SqlitePool = Pool<Sqlite>;

lazy_static! {
    static ref SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::const_new();
}

async fn get_global_pool() -> &'static SqlitePool {
    SQLITE_POOL
        .get_or_init(|| async {
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect("./confidant.sqlite?mode=rwc")
                .await
                .map_err(|e| {
                    error!(message = "数据库连接池中获取链接失败", error = ?e);
                    e
                })
                .unwrap()
        })
        .await
}

#[derive(Debug, Serialize)]
pub(super) struct ScaleStatistics<'a> {
    name: &'a str,
    times: i32,
}

#[derive(sqlx::Type, Default, Debug, Clone, Copy)]
#[repr(i32)]
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
            _ => {
                error!(message = "无效的客户端类型", value = value);
                Err(MindPulseError::InvalidClientType(value))
            }
        }
    }
}

pub async fn create_table() -> MindPulseResult<()> {
    trace!(message = "创建表格");

    let pool = get_global_pool().await;

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
        error!(message = "创建表格失败", error = ?e);
        e
    })?;

    info!(
        message = "已创建表格（如果表格不存在）",
        table = "statistics_ip"
    );

    Ok(())
}

pub(super) async fn select_one(scale: &str) -> MindPulseResult<ScaleStatistics<'_>> {
    trace!(message = "正在查询测试统计", scale = scale);

    let pool = get_global_pool().await;

    let scale_index = get_scale_index_by_path(scale)?;
    debug!(message = "测试索引", index = scale_index, scale = scale);

    debug!(message = "查询统计次数", scale = scale);
    let (times,): (i32,) =
        sqlx::query_as("SELECT COUNT(*) as times FROM statistics_ip WHERE scale_index = $1")
            .bind(scale_index as i32)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                error!(message = "查询单条结果出错", scale = scale, error = ?e);
                e
            })?;

    info!(message = "查询到统计次数", scale = scale, times = times);

    Ok(ScaleStatistics { name: scale, times })
}

pub(super) async fn select_all<'a>() -> MindPulseResult<Vec<ScaleStatistics<'a>>> {
    let pool = get_global_pool().await;

    trace!(message = "获取全部测试的统计次数");
    let rows: Vec<(i32, i32)> = sqlx::query_as(
        "select scale_index, count(*) as times
from statistics_ip
group by scale_index;",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(message = "获取全部 statistics 失败", error = ?e);
        e
    })?;

    let mut scale_statistics_list: Vec<ScaleStatistics<'_>> = PATHS
        .iter()
        .map(|p| ScaleStatistics {
            name: p.name(),
            times: 0,
        })
        .collect();

    for (idx, times) in rows.iter() {
        scale_statistics_list[*idx as usize].times = *times;
    }

    info!(message = "已获取到全部测试的统计次数", list = ?scale_statistics_list);

    Ok(scale_statistics_list)
}

pub(super) async fn insert_one_finised_test(
    scale_index: usize,
    client_type: u8,
    addr: &str,
) -> MindPulseResult<u64> {
    trace!(message = "转换客户端类型", client_type = client_type);
    let client_type: ClientType = client_type.try_into().map_err(|e| {
        error!(message = "转换客户端类型失败", error = ?e);
        e
    })?;
    debug!(message = "成功转换客户端类型", client_type = ?client_type);

    trace!(message = "获取当前时间");
    let now = time::OffsetDateTime::now_utc().to_offset(offset!(+8));
    debug!(message = "已获取到当前时间", now = ?now);

    let pool = get_global_pool().await;

    trace!(
        message = "插入记录",
        scale_index = scale_index,
        addr = addr,
        client_type = ?client_type
    );
    let result = sqlx::query(
        "INSERT INTO statistics_ip (scale_index, ip, finished_time, client_type) VALUES ($1, $2, $3, $4)",
    )
    .bind(scale_index as i32)
    .bind(addr)
    .bind(now)
    .bind(client_type)
    .execute(pool)
    .await
    .map_err(|e| {
        error!(message = "插入单条 SQL 失败", error = ?e, scale_index = scale_index, client_type = ?client_type);
        e
    })?;

    info!(
        message = "已插入一条测试记录",
        scale_index = scale_index,
        addr = addr,
        client_type = ?client_type
    );

    Ok(result.rows_affected())
}
