mod sqlite;

use salvo::{handler, http::StatusCode, writing::Json, Request, Response};

use crate::{error::MindPulseResult, scale::get_scale_index_by_path};

use self::sqlite::{insert_one_finised_test, select_all, select_one};

pub use self::sqlite::create_table;

fn get_scale_query(req: &Request) -> Option<String> {
    req.query("scale")
}

fn get_client_type(req: &Request) -> Option<u8> {
    req.query("clientType")
}

#[handler]
pub(super) async fn get_statistics(req: &Request, res: &mut Response) -> MindPulseResult<()> {
    trace!(message = "查询统计数次");
    let scale = get_scale_query(req);

    match scale {
        None => {
            debug!(message = "未获取到 scale，查询全部记录");
            res.render(Json(select_all().await?));
        }
        Some(s) => {
            debug!(message = "获取到 scale，查询单条记录", scale = s);
            res.render(Json(select_one(&s).await?));
        }
    };

    Ok(())
}

#[handler]
pub(super) async fn insert_statistics_ip(req: &Request, res: &mut Response) -> MindPulseResult<()> {
    trace!(message = "插入一条测试记录");

    let scale = get_scale_query(req);
    let scale = match scale {
        None => {
            error!(message = "scale 无效", scale = scale);
            res.status_code(StatusCode::BAD_REQUEST);
            return Err("scale 无效".into());
        }
        Some(s) => s,
    };

    let client_type = match get_client_type(req) {
        None => {
            error!(message = "client_type 无效");
            res.status_code(StatusCode::BAD_REQUEST);
            return Err("client_type 无效".into());
        }
        Some(n) => n,
    };

    let scale_index = get_scale_index_by_path(&scale)?;
    debug!(
        message = "获取到 scale 索引",
        scale = scale,
        index = scale_index
    );

    let client_ip: &str = req.header("X-Forwarded-For").unwrap_or("");
    debug!(message = "当前请求客户端的 IP", addr = client_ip);

    insert_one_finised_test(scale_index, client_type, client_ip).await?;

    Ok(())
}
