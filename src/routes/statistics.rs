use salvo::oapi::extract::QueryParam;
use salvo::prelude::*;
use salvo::writing::Json;

use crate::error::{Error, Result};
use crate::routes::AppState;
use crate::scale::ScaleId;
use crate::statistics::{self, ClientType};

/// 提取真实客户端 IP（取 X-Forwarded-For 的第一段）
fn client_ip(x_forwarded_for: Option<&str>) -> &str {
    x_forwarded_for
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .unwrap_or("")
}

fn state(depot: &Depot) -> Result<&AppState> {
    depot
        .get_typed::<AppState>()
        .map_err(|_| Error::StateMissing)
}

/// GET /statistics?id=&client_type=
#[handler]
pub async fn record(
    id: QueryParam<u16, true>,
    client_type: QueryParam<u8, true>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<()> {
    trace!(
        message = "Inserting test record",
        id = *id,
        client_type = *client_type
    );

    // 立即转为自有 String，避免把请求头的借用带过 await
    let forwarded = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok());
    let client_ip = client_ip(forwarded).to_owned();
    debug!(message = "Retrieved client IP address", ip = client_ip);

    let pool = &state(depot)?.pool;
    let client_type = ClientType::try_from(*client_type)?;

    statistics::record(pool, ScaleId(*id), client_type, &client_ip).await?;

    Ok(())
}

/// GET /get_statistics?id=
#[handler]
pub async fn query(
    id: QueryParam<u16, false>,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<()> {
    trace!(message = "Querying statistics data");
    let pool = &state(depot)?.pool;

    match id.into_inner() {
        None => {
            debug!(message = "No scale specified, querying all records");
            res.render(Json(statistics::all_counts(pool).await?));
        }
        Some(id) => {
            debug!(message = "Querying specified scale record", id);
            res.render(Json(statistics::scale_count(pool, ScaleId(id)).await?));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::client_ip;

    #[test]
    fn takes_first_forwarded_address() {
        assert_eq!(client_ip(Some("1.2.3.4, 5.6.7.8")), "1.2.3.4");
        assert_eq!(client_ip(Some("  9.9.9.9  ")), "9.9.9.9");
    }

    #[test]
    fn empty_or_missing_header_yields_empty_ip() {
        assert_eq!(client_ip(None), "");
        assert_eq!(client_ip(Some("")), "");
    }
}
