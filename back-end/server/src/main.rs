#[macro_use]
extern crate rocket;

pub mod catchers;
pub mod handlers;
pub mod models;
pub mod mongodb_driver;

use rocket::{http::Status, response::content::Json};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .register(catchers![
            crate::catchers::not_found,
            crate::catchers::internal_error
        ])
        .manage(crate::mongodb_driver::init_db().await)
        .mount("/", routes![index])
        .mount("/users", routes![crate::handlers::user::delete_all])
        .mount(
            "/api/auth",
            routes![crate::handlers::auth::login, crate::handlers::auth::logout],
        )
}
