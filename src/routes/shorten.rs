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

    // Only check for existence if encryption is off.
    if !req.encrypt {
        if let Ok(existing) = find_by_original_url(&mut conn, original_url) {
            return Ok(HttpResponse::Ok().json(existing));
        }
    }

    // Try inserting, with a limited number of retries for collision handling.
    const MAX_RETRIES: usize = 5;
    let mut attempt = 0;
    loop {
        let code = random_short_code(6);
        let mut new_short = NewShortUrl {
            original_url,
            short_code: &code,
            encrypted: req.encrypt,
            transaction_hash: req.transaction_hash.as_deref(),
        };

        match insert_short_url(&mut conn, &mut new_short) {
            Ok(saved) => return Ok(HttpResponse::Ok().json(saved)),
            Err(_e) if attempt < MAX_RETRIES => {
                // Optionally, check if the error is a duplicate key error.
                attempt += 1;
            }
            Err(e) => {
                return Err(actix_web::error::ErrorBadRequest(e.to_string()));
            }
        }
    }
}
