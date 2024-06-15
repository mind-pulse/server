use std::collections::HashMap;

use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use time::macros::offset;
use tokio::sync::OnceCell;

use crate::{
    error::{MindPulseError, MindPulseResult},
    scale::{get_scale_index_by_path, get_scale_name_by_path, get_scale_path_by_name, PATHS},
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

async fn select_one_from_old_table<'a>(
    pool: &SqlitePool,
    scale: &'a str,
) -> MindPulseResult<ScaleStatistics<'a>> {
    trace!(message = "正在旧表中查询一条记录", scale = scale);

    let row: (i32,) = sqlx::query_as("SELECT times FROM statistics WHERE scale = $1")
        .bind(scale)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!(message = "单条结果中获取 times 失败", error = ?e, scale = scale);
            e
        })?;

    debug!(message = "旧表中查询到记录", scale = scale, times = row.0);

    let name = get_scale_name_by_path(scale)?;

    Ok(ScaleStatistics { name, times: row.0 })
}

pub(super) async fn select_one(scale: &str) -> MindPulseResult<ScaleStatistics<'_>> {
    trace!(message = "正在查询测试统计", scale = scale);

    let pool = get_global_pool().await;

    let scale_index = get_scale_index_by_path(scale)?;
    debug!(message = "测试索引", index = scale_index);

    let mut statistics = select_one_from_old_table(pool, scale).await?;
    info!(
        message = "旧表中查询到统计结果",
        scale = statistics.name,
        times = statistics.times
    );

    debug!(message = "新表中查询统计次数", scale = scale);

    let row: (i32,) =
        sqlx::query_as("SELECT COUNT(*) as times FROM statistics_ip WHERE scale_index = $1")
            .bind(scale_index as i32)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                error!(message = "查询单条结果出错", scale = scale, error = ?e);
                e
            })?;

    debug!(
        message = "新表中查询到统计次数",
        scale = scale,
        times = row.0
    );

    statistics.times += row.0;

    info!(
        message = "查询到统计次数",
        scale = scale,
        times = statistics.times
    );

    Ok(statistics)
}

async fn select_all_from_old_table<'a>(pool: &SqlitePool) -> MindPulseResult<HashMap<String, i32>> {
    trace!(message = "正在旧表中查询全部测试的统计次数");

    let rows: Vec<(String, i32)> = match sqlx::query_as("SELECT scale, times FROM statistics")
        .fetch_all(pool)
        .await
    {
        Ok(rows) => rows,
        Err(err) => match &err {
            sqlx::Error::Database(e) => {
                let code = e.code();
                if code == Some(std::borrow::Cow::Borrowed("1")) {
                    warn!(message = e.message(), code = ?e.code());
                    vec![]
                } else {
                    error!(message = "获取全部 statistics 失败", error = ?e);
                    return Err(err.into());
                }
            }
            _ => {
                error!(message = "获取全部 statistics 失败", error = ?err);
                return Err(err.into());
            }
        },
    };

    debug!(message = "已在旧表中获取到全部统计次数", rows = ?rows);

    Ok(HashMap::from_iter(rows))
}

pub(super) async fn select_all<'a>() -> MindPulseResult<Vec<ScaleStatistics<'a>>> {
    let pool = get_global_pool().await;

    let old_scale_statistics_list = select_all_from_old_table(pool).await?;

    debug!(
        message = "已获取到旧表中的统计结果 HashMap",
        old_list = ?old_scale_statistics_list,
    );

    trace!(message = "获取新表中全部测试的统计次数");
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

    debug!(message = "已获取新表中全部测试的统计次数", rows = ?rows);

    trace!(message = "初始化 Vec<ScaleStatistics>");
    let mut scale_statistics_list: Vec<ScaleStatistics<'_>> = PATHS
        .iter()
        .map(|p| ScaleStatistics {
            name: p.name(),
            times: 0,
        })
        .collect();
    debug!(message = "已初始化 Vec<ScaleStatistics>", list = ?scale_statistics_list);

    trace!(message = "新表中的查询结果合并到 Vec<ScaleStatistics>");
    for (idx, times) in rows.iter() {
        scale_statistics_list[*idx as usize].times = *times;
    }
    debug!(message = "已将新表的查询结果合并到 Vec<ScaleStatistics>", list = ?scale_statistics_list);

    trace!(message = "合并旧表和新表的查询结果");
    for item in scale_statistics_list.iter_mut() {
        let path = get_scale_path_by_name(item.name)?;
        item.times += old_scale_statistics_list.get(path).unwrap_or(&0);
    }
    debug!(message = "旧表和新表的查询结果已合并", list = ?scale_statistics_list);

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
