// TODO: remove AUTH_SECRET_KEY from .env file
// and change DATABASE_URL to example format
#![feature(async_closure)]
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};

use crate::mongodb_driver::Connection;

mod errors;
mod handlers;
mod models;
mod mongodb_driver;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    // Init logger
    pretty_env_logger::init();

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Init mongodb
    let db_conn = Connection::new().await;

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(db_conn.clone())
            .wrap(middleware::Logger::default()) // enable logger
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400) // one day in seconds
                    .secure(false), // this can only be true if you have https
            ))
            .data(web::JsonConfig::default().limit(4096))
        .service(
            web::scope("/api")
                .service(
                    web::resource("/auth")
                        .route(web::post().to(crate::handlers::api::auth::handler::login))
                        .route(web::delete().to(crate::handlers::api::auth::handler::logout))
                        .route(web::get().to(crate::handlers::api::auth::handler::get_me)),
                )
                .service(
                    web::resource("/schedule").route(
                        web::get()
                            .to(crate::handlers::api::schedule::get_schedule_handler::get_schedule),
                    ),
                ),
            // .service(web::resource("/user").route(web::post().to(unimplemented!()))),
        )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
