mod catalog;
mod statistics;

use salvo::prelude::*;

use crate::db::SqlitePool;

/// 通过 Depot 注入到每次请求的共享状态
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

/// 把 `AppState` 放进 Depot 的中间件
pub struct InjectState(pub AppState);

#[async_trait]
impl Handler for InjectState {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        depot.insert_typed(self.0.clone());
        ctrl.call_next(req, depot, res).await;
    }
}

/// 构建路由表
pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("version").get(catalog::version))
        .push(Router::with_path("list").get(catalog::list))
        .push(
            Router::with_path("scales")
                .get(catalog::list)
                .push(Router::with_path("{id}").get(catalog::item)),
        )
        .push(Router::with_path("statistics").get(statistics::record))
        .push(Router::with_path("get_statistics").get(statistics::query))
}
