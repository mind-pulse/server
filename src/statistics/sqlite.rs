use time::macros::offset;

use crate::db::SqlitePool;
use crate::statistics::ClientType;

/// 各量表的测试次数
pub async fn counts_by_scale(pool: &SqlitePool) -> Result<Vec<(u16, u64)>, sqlx::Error> {
    // scale_index 为旧版量表的索引，现在已改为 id
    sqlx::query_as(
        "SELECT scale_index, COUNT(*) as count
         FROM statistics_ip
         GROUP BY scale_index",
    )
    .fetch_all(pool)
    .await
}

/// 单个量表的测试次数
pub async fn count_for_scale(pool: &SqlitePool, id: u16) -> Result<u64, sqlx::Error> {
    let (count,): (u64,) =
        sqlx::query_as("SELECT COUNT(*) as times FROM statistics_ip WHERE scale_index = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(count)
}

/// 插入一条完成的测试记录，返回受影响行数
pub async fn insert(
    pool: &SqlitePool,
    id: u16,
    client_type: ClientType,
    ip_address: &str,
) -> Result<u64, sqlx::Error> {
    trace!(
        message = "Inserting test record",
        scale_index = id,
        ip_address,
        client_type = ?client_type
    );

    let timestamp = time::OffsetDateTime::now_utc().to_offset(offset!(+8));

    // 为了兼容旧表，id 列名暂时仍使用 scale_index
    let result = sqlx::query(
        "INSERT INTO statistics_ip (scale_index, ip, finished_time, client_type) VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(ip_address)
    .bind(timestamp)
    .bind(client_type)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
