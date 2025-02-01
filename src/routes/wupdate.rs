// src/routes/wupdate.rs

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::db::operations::decrement_tries_left;
use crate::Pool; // type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Request body for decrementing `tries_left`
#[derive(Deserialize)]
pub struct UpdateWalletRequest {
    pub wallet_address: String,
}

/// POST /wupdate
/// Decrements the `tries_left` value by 1 for the given wallet (if above 0).
#[post("/wupdate")]
pub async fn decrement_wallet_route(
    pool: web::Data<Pool>,
    req: web::Json<UpdateWalletRequest>,
) -> actix_web::Result<HttpResponse> {
    // Acquire a DB connection from the pool
    let mut conn = pool
        .get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Call our Diesel operation to decrement tries_left
    let updated_wallet = decrement_tries_left(&mut conn, &req.wallet_address)
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;

    // Return the updated record as JSON
    Ok(HttpResponse::Ok().json(updated_wallet))
}
