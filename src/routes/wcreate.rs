// src/routes/wallet.rs

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::db::operations::{create_wallet, find_by_wallet_address};
use crate::Pool; // type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Request body for creating/registering a new wallet
#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub wallet_address: String,
}

/// POST /wcreate
/// Creates a new wallet entry with an initial number of tries.
#[post("/wcreate")]
pub async fn create_wallet_route(
    pool: web::Data<Pool>,
    req: web::Json<CreateWalletRequest>,
) -> actix_web::Result<HttpResponse> {
    // Acquire a DB connection from the pool
    let mut conn = pool
        .get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // 1. Check if original_url exists
    if let Ok(existing) = find_by_wallet_address(&mut conn, &req.wallet_address) {
        // If found, just return the existing short_code
        return Ok(HttpResponse::Ok().json(existing));
    }

    // Call our Diesel operation
    let wallet = create_wallet(
        &mut conn, 
        &req.wallet_address, 
    )
    .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;

    // Return the newly created record as JSON
    Ok(HttpResponse::Ok().json(wallet))
}
