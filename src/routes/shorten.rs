// src/routes/create_short_url.rs

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::db::operations::{find_by_original_url, insert_short_url};
use crate::db::models::NewShortUrl;
use crate::utils::rcode::random_short_code; // We'll define a "utils" mod for random code
use crate::Pool; // We'll define a type alias for DB pool in main or lib

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub original_url: String,
    pub encrypt: bool,
    pub transaction_hash: Option<String>,
}

#[post("/shorten")]
pub async fn create_short_url(
    pool: web::Data<Pool>,
    req: web::Json<ShortenRequest>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("DB pool error: {e}"))
    })?;

    let original_url = req.original_url.trim();

    // 1. Check if original_url exists (in plain or cipher?)
    if !req.encrypt {
        if let Ok(existing) = find_by_original_url(&mut conn, original_url) {
            // If found, just return the existing
            return Ok(HttpResponse::Ok().json(existing));
        }
    }

    // 2. Otherwise, generate short_code
    let code = random_short_code(6);

    // 3. Insert new record into DB
    let mut new_short = NewShortUrl {
        original_url,
        short_code: &code,
        encrypted: req.encrypt,
        transaction_hash: req.transaction_hash.as_deref(),
    };

    match insert_short_url(&mut conn, &mut new_short) {
        Ok(saved) => Ok(HttpResponse::Ok().json(saved)),
        Err(e) => {
            // If there's a duplicate key error, you could keep trying or return an error
            Err(actix_web::error::ErrorBadRequest(e.to_string()))
        }
    }
}