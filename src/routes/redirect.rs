// src/routes/redirect.rs
use actix_web::{get, web, HttpResponse, http::header};
use crate::db::operations::{find_by_short_code, mark_as_expired};
use crate::Pool;
use crate::utils::crypto::decrypt_url;

#[get("/{short_code}")]
pub async fn redirect_short(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let code = path.into_inner();

    let mut conn = pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("DB pool error: {e}"))
    })?;

    let result = find_by_short_code(&mut conn, &code);
    match result {
        Ok(record) => {
            // If record.expired, redirect user to main page:
            if record.expired {
                return Ok(HttpResponse::Found()
                    .append_header((header::LOCATION, "/"))
                    .finish());
            }
            // If user paid => check time since creation
            if record.transaction_hash.is_some() {
                use chrono::Utc;
                let now = Utc::now().naive_utc();
                let hours_since_create = (now - record.created_at).num_hours();

                if hours_since_create >= 24 {
                    // 24 hours or more have passed => set expired + redirect to main page
                    mark_as_expired(&mut conn, record.id);
                    return Ok(HttpResponse::Found()
                        .append_header((header::LOCATION, "/"))
                        .finish());
                }
            }
            // If record is encrypted, we decrypt
            let target_url = if record.encrypted {
                match decrypt_url(&record.original_url) {
                    Ok(decrypted) => decrypted,
                    Err(_) => {
                        // If decrypt fails, you can decide how to respond
                        return Ok(HttpResponse::BadRequest().body("Invalid encrypted data"));
                    }
                }
            } else {
                record.original_url
            };
            // 301/302 redirect to the original_url
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, target_url))
                .finish())
        }
        Err(_) => Ok(HttpResponse::NotFound().body("Short URL not found.")),
    }
}
