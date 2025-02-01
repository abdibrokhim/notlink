// src/routes/expire_short_url.rs (example new route)
use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::db::operations::mark_as_expired_if_paid;
use crate::Pool;

#[derive(Deserialize)]
pub struct ExpireRequest {
    pub short_url_id: i32,
}

#[post("/expire")]
pub async fn expire_short_url(
    pool: web::Data<Pool>,
    req: web::Json<ExpireRequest>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool
        .get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("DB pool error: {e}")))?;

    match mark_as_expired_if_paid(&mut conn, req.short_url_id) {
        Ok(updated) => Ok(HttpResponse::Ok().json(updated)),
        Err(e) => {
            // e.g. If there's no transaction_hash, we fail
            Err(actix_web::error::ErrorBadRequest(e.to_string()))
        }
    }
}
