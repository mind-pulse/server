use std::path::Path;

use sqlx::sqlite::SqlitePoolOptions;

/// 全局共用的 SQLite 连接池类型
pub type SqlitePool = sqlx::Pool<sqlx::Sqlite>;

/// 创建 SQLite 连接池（库文件不存在时自动创建）
pub async fn connect(path: &Path) -> Result<SqlitePool, sqlx::Error> {
    info!(message = "Using database path", path = %path.display());

    let url = format!("{}?mode=rwc", path.display());
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
}

/// 执行建表迁移
pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    trace!(message = "Creating statistics table");

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
    .await?;

    info!(
        message = "Statistics table created or already exists",
        table = "statistics_ip"
    );

    Ok(())
}
