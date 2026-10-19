use std::net::SocketAddrV4;

use salvo::prelude::*;

use crate::access_log::AccessLog;
use crate::db::SqlitePool;
use crate::routes::{self, AppState, InjectState};

/// 启动 HTTP 服务
pub async fn serve(pool: SqlitePool, address: SocketAddrV4) {
    let router = routes::router().hoop(InjectState(AppState { pool }));
    let service = Service::new(router).hoop(AccessLog);

    info!(message = "Server running", %address);

    let listener = TcpListener::new(address).bind().await;
    Server::new(listener).serve(service).await;
}
