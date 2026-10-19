mod sqlite;
mod types;

use std::collections::HashMap;

use crate::db::SqlitePool;
use crate::error::Result;
use crate::scale::{self, ScaleId};

pub use types::{ClientType, ScaleStatistics};

/// 每个量表的测试次数（包含暂无记录、计数为 0 的量表）
pub async fn all_counts(pool: &SqlitePool) -> Result<HashMap<u16, ScaleStatistics<'static>>> {
    trace!(message = "Querying all scale statistics");

    let rows = sqlite::counts_by_scale(pool).await?;

    // 先用注册表补齐所有量表，再用真实计数覆盖
    let mut statistics: HashMap<u16, ScaleStatistics<'static>> = scale::all_names()
        .into_iter()
        .map(|(ScaleId(id), name)| (id, ScaleStatistics { name, count: 0 }))
        .collect();

    for (id, count) in rows {
        if let Some(stats) = statistics.get_mut(&id) {
            stats.count = count;
        }
    }

    info!(message = "All scale statistics retrieved", statistics = ?statistics);
    Ok(statistics)
}

/// 单个量表的测试次数
pub async fn scale_count(pool: &SqlitePool, id: ScaleId) -> Result<ScaleStatistics<'static>> {
    trace!(message = "Querying scale statistics", id = id.0);

    let name = scale::name(id)?;
    let count = sqlite::count_for_scale(pool, id.0).await?;

    info!(message = "Scale statistics retrieved", id = id.0, count);
    Ok(ScaleStatistics { name, count })
}

/// 记录一次完成的测试
pub async fn record(
    pool: &SqlitePool,
    id: ScaleId,
    client_type: ClientType,
    ip_address: &str,
) -> Result<u64> {
    // 校验量表 ID 是否存在
    scale::name(id)?;

    let affected = sqlite::insert(pool, id.0, client_type, ip_address).await?;

    info!(
        message = "Test record inserted successfully",
        scale_index = id.0,
        ip_address,
        client_type = ?client_type
    );
    Ok(affected)
}
