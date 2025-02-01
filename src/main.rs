// #[macro_use]

use actix_web::{get, web, middleware::Logger};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::{CustomError, SecretStore};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub mod db;
pub mod routes;
pub mod utils; // <--- so we can call get_pg_url()

// We'll alias this for convenience
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello_world() -> &'static str {
    "this is notlink - a new kind of url shortener"
}

#[shuttle_runtime::main]
async fn actix_main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    // 0) Initialize secrets from Shuttle SecretStore
    utils::keys::init_secrets(&secrets);

    // 1) Get DB URL from your keys.rs (which reads from .env)
    let database_url = utils::keys::get_pg_url();
    
    // 2) Create a Diesel r2d2 pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let diesel_pool = r2d2::Pool::builder()
        .max_size(5) // or higher if you prefer
        .build(manager)
        .map_err(|e| CustomError::new(e))?;

    // 3) Set up your Actix configuration
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            // This empty string "" means we apply it at the root.
            web::scope("")
                .wrap(Logger::default())
                .app_data(web::Data::new(diesel_pool.clone()))
                .service(hello_world)
                .service(routes::create_short_url)
                .service(routes::redirect_short)
                .service(routes::expire_short_url)
                .service(routes::get_all_short_urls_route)
                .service(routes::create_wallet_route)
                .service(routes::decrement_wallet_route)
        );
    };

    // 4) Return the config as Actix service
    Ok(config.into())
}
