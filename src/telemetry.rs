use time::macros::{format_description, offset};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

/// 初始化日志。
///
/// 返回的 `WorkerGuard` 必须由调用方持有到进程结束，否则异步日志会丢失。
pub fn init() -> WorkerGuard {
    let timer = OffsetTime::new(
        offset!(+8),
        if cfg!(debug_assertions) {
            format_description!("[hour]:[minute]:[second].[subsecond digits:3]")
        } else {
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            )
        },
    );

    // NOTE: _guard must be a top-level variable
    let (writer, guard) = {
        let file_appender = tracing_appender::rolling::daily("./log", "confidant-server.log");
        tracing_appender::non_blocking(file_appender)
    };

    #[cfg(debug_assertions)]
    let writer = {
        use tracing_subscriber::fmt::writer::MakeWriterExt;
        std::io::stderr.and(writer)
    };

    let log_level = if cfg!(debug_assertions) {
        Level::TRACE
    } else {
        Level::WARN
    };

    let builder = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_env_filter("server")
        .with_timer(timer)
        .with_writer(writer);

    #[cfg(debug_assertions)]
    builder.init();

    #[cfg(not(debug_assertions))]
    builder.json().init();

    guard
}
