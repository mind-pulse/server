use salvo::oapi::extract::PathParam;
use salvo::prelude::*;
use salvo::writing::Json;

use crate::error::Result;
use crate::scale::{self, ScaleId};

/// GET /version
#[handler]
pub async fn version(res: &mut Response) {
    res.render(env!("CARGO_PKG_VERSION"));
}

/// GET /list | GET /scales
#[handler]
pub async fn list(res: &mut Response) {
    res.render(Json(scale::summaries()));
}

/// GET /scales/{id}
#[handler]
pub async fn item(id: PathParam<u16>, res: &mut Response) -> Result<()> {
    res.render(Json(scale::json(ScaleId(*id))?));
    Ok(())
}
