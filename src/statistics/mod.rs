mod sqlite;

use salvo::{
    handler,
    oapi::extract::{HeaderParam, QueryParam},
    writing::Json,
    Response, Writer,
};

use crate::{
    error::{MindPulseError, MindPulseResult},
    scale::get_scale_name_by_id,
    statistics::sqlite::ClientType,
};

use self::sqlite::{insert_completed_test, query_all_statistics, query_scale_statistics};

pub use self::sqlite::create_statistics_table;

/// Render statistics as JSON for all scales or for a specific scale.
///
/// If `id` is `None`, responds with JSON containing all recorded statistics. If `id` is `Some(id)`, responds with JSON containing statistics for the specified scale `id`.
///
/// # Examples
///
/// ```
/// # use salvo::oapi::extract::QueryParam;
/// # use salvo::Response;
/// # async fn _example() {
/// let mut res = Response::new();
/// let id: QueryParam<u16, false> = QueryParam::none();
/// let _ = crate::statistics::handle_get_statistics(id, &mut res).await;
/// # }
/// ```
#[handler]
pub async fn handle_get_statistics(
    id: QueryParam<u16, false>,
    res: &mut Response,
) -> MindPulseResult<()> {
    trace!(message = "Querying statistics data");

    match id.into_inner() {
        None => {
            debug!(message = "No scale specified, querying all records");
            res.render(Json(query_all_statistics().await?));
        }
        Some(id) => {
            debug!(message = "Querying specified scale record", id);
            res.render(Json(query_scale_statistics(id).await?));
        }
    };

    Ok(())
}

/// Insert a completed test record for a scale after validating inputs and extracting the client's IP.
///
/// This handler resolves the scale name for `id`, converts `client_type` into the internal `ClientType`
/// (returning a `MindPulseError::Response` with message `"无效的 clientType"` if conversion fails),
/// derives the client IP from the first entry of the `X-Forwarded-For` header, and persists the record.
/// Returns `Ok(())` on success or an error from validation or insertion operations.
///
/// # Examples
///
/// ```
/// use salvo::oapi::extract::{QueryParam, HeaderParam};
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Example invocation — in real handlers these extractors are provided by the framework.
/// let id = QueryParam::from(1u16);
/// let client_type = QueryParam::from(0u8);
/// let xff = HeaderParam::from(Some("203.0.113.5"));
/// let _ = crate::statistics::handle_insert_record(id, client_type, xff).await;
/// # Ok(()) }
/// ```
#[handler]
pub async fn handle_insert_record(
    id: QueryParam<u16, true>,
    client_type: QueryParam<u8, true>,
    x_forwarded_for: HeaderParam<&str, false>,
) -> MindPulseResult<()> {
    trace!(
        message = "Inserting test record",
        id = *id,
        client_type = *client_type
    );

    // 验证 ID
    let name = get_scale_name_by_id(*id)?;
    debug!(
        message = "Successfully resolved scale index",
        name = name,
        id = *id
    );

    // 验证 client type
    let client_type: ClientType = (*client_type).try_into().map_err(|e| {
        error!(message = "Failed to convert client type", error = ?e);
        MindPulseError::Response("无效的 clientType".to_owned())
    })?;

    // 获取客户端 IP 地址
    let client_ip = x_forwarded_for
        .into_inner()
        .unwrap_or("")
        .split(',')
        .next()
        .unwrap_or("")
        .trim();
    debug!(message = "Retrieved client IP address", ip = client_ip);

    // 插入测试记录
    insert_completed_test(*id, client_type, client_ip).await?;

    Ok(())
}