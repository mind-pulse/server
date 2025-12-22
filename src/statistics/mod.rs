mod sqlite;

use salvo::{handler, http::StatusCode, writing::Json, Request, Response};

use crate::{error::MindPulseResult, scale::get_scale_id_by_path};

use self::sqlite::{insert_completed_test, query_all_statistics, query_scale_statistics};

pub use self::sqlite::create_statistics_table;

/// 从请求参数中获取量表路径
fn extract_scale_param(req: &Request) -> Option<String> {
    req.query("scale")
}

/// 从请求参数中获取客户端类型
fn extract_client_type(req: &Request) -> Option<u8> {
    req.query("clientType")
}

/// 获取查询统计信息的处理器
#[handler]
pub(super) async fn handle_get_statistics(
    req: &Request,
    res: &mut Response,
) -> MindPulseResult<()> {
    trace!(message = "Querying statistics data");

    let scale = extract_scale_param(req);

    match scale {
        None => {
            debug!(message = "No scale specified, querying all records");
            res.render(Json(query_all_statistics().await?));
        }
        Some(scale_path) => {
            debug!(
                message = "Querying specified scale record",
                scale = scale_path
            );
            res.render(Json(query_scale_statistics(&scale_path).await?));
        }
    };

    Ok(())
}

/// 处理插入测试记录的请求
#[handler]
pub(super) async fn handle_insert_record(req: &Request, res: &mut Response) -> MindPulseResult<()> {
    trace!(message = "Inserting test record");

    // 获取并验证量表路径参数
    let scale_path = match extract_scale_param(req) {
        None => {
            error!(message = "Missing required scale parameter");
            res.status_code(StatusCode::BAD_REQUEST);
            return Err("Missing scale parameter".into());
        }
        Some(path) => path,
    };

    // 获取并验证客户端类型参数
    let client_type = match extract_client_type(req) {
        None => {
            error!(message = "Missing required client_type parameter");
            res.status_code(StatusCode::BAD_REQUEST);
            return Err("Missing client_type parameter".into());
        }
        Some(ty) => ty,
    };

    // 解析量表 ID
    let id = get_scale_id_by_path(&scale_path)?;
    debug!(
        message = "Successfully resolved scale index",
        scale = scale_path,
        id = id
    );

    // 获取客户端 IP 地址
    let client_ip = req.header("X-Forwarded-For").unwrap_or("").trim();
    debug!(message = "Retrieved client IP address", ip = client_ip);

    // 插入测试记录
    insert_completed_test(id, client_type, client_ip).await?;

    Ok(())
}
