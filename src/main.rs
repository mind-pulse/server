mod error;
mod logger;
mod scale;
mod statistics;

#[macro_use]
extern crate tracing;

use salvo::oapi::extract::PathParam;
use salvo::prelude::*;
use salvo::writing::Json;
use time::macros::{format_description, offset};
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::error::MindPulseResult;
use crate::logger::Logger;
use crate::scale::{get_scale_json_by_id, LIST};
use crate::statistics::{create_statistics_table, handle_get_statistics, handle_insert_record};

trait JsonRender {
    fn json<S>(&mut self, data: S)
    where
        S: serde::Serialize + std::marker::Send;
}

impl JsonRender for Response {
    fn json<S>(&mut self, data: S)
    where
        S: serde::Serialize + std::marker::Send,
    {
        self.render(Json(data))
    }
}

/// Renders the public list of scales (`LIST`) as JSON into the given response.
///
/// # Examples
///
/// ```
/// // inside an async handler
/// list(res).await;
/// ```
#[handler]
async fn list(res: &mut Response) {
    res.json(LIST);
}

/// Retrieves the scale identified by `id` and writes its JSON representation into the response.
///
/// The handler obtains the scale JSON for the given `id` and renders it into `res`.
///
/// # Returns
///
/// `Ok(())` on success; an error if the scale cannot be found or if retrieval/serialization fails.
///
/// # Examples
///
/// ```
/// // Conceptual usage within a router: a request to GET /scales/3 results in calling:
/// // item(PathParam::from(3u16), &mut response).await;
/// ```
#[handler]
async fn item(id: PathParam<u16>, res: &mut Response) -> MindPulseResult<()> {
    let value = get_scale_json_by_id(*id)?;
    res.json(value);

    Ok(())
}

/// Starts the HTTP server bound to 127.0.0.1 at the specified TCP port.
///
/// This configures the router and service (including the request logger), binds a TCP
/// listener to 127.0.0.1:<port>, and runs the Salvo server until the process exits.
///
/// # Examples
///
/// ```
/// // Run the server in the background within a Tokio runtime.
/// // Note: in real usage this call typically runs for the lifetime of the process.
/// tokio::spawn(async {
///     serve(4819).await;
/// });
/// ```
async fn serve(port: u16) {
    let router = Router::new()
        .push(Router::with_path("list").get(list))
        .push(
            Router::with_path("scales")
                .get(list)
                .push(Router::with_path("{id}").get(item)),
        )
        .push(Router::with_path("statistics").get(handle_insert_record))
        .push(Router::with_path("get_statistics").get(handle_get_statistics));

    let service = Service::new(router).hoop(Logger);

    let address = format!("127.0.0.1:{}", port);
    info!("Server running on {}", address);

    let listener = TcpListener::new(address).bind().await;
    Server::new(listener).serve(service).await;
}

#[tokio::main]
async fn main() -> MindPulseResult<()> {
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
    let (writer, _guard) = {
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

    create_statistics_table().await?;

    // 解析命令行参数获取端口号，默认为 4819
    let port = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<u16>().ok())
        .unwrap_or(4819);

    serve(port).await;

    Ok(())
}