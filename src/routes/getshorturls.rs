use actix_web::{get, web, HttpResponse};
use crate::db::operations::get_all_short_urls;
use crate::Pool;

#[get("/getshorturls")]
pub async fn get_all_short_urls_route(
    pool: web::Data<Pool>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool.get()
        .map_err(|err| actix_web::error::ErrorInternalServerError(format!("DB pool error: {err}")))?;

    match get_all_short_urls(&mut conn) {
        Ok(urls) => {
            if urls.is_empty() {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Ok(HttpResponse::Ok().json(urls))
            }
        },
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e.to_string())),
    }
}
