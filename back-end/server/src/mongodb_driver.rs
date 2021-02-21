use dotenv::dotenv;
use mongodb::{Client, Database};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::env;
use std::ops::Deref;

pub struct Conn(pub Database);

// TODO: make normal erro
#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request
            .guard::<State<Database>>()
            .await
            .map(|db| Conn(db.inner().clone())) // TODO: Is it normal to use .clone()?
    }
}
impl Deref for Conn {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
pub async fn init_db() -> Database {
    dotenv().ok();
    let mongodb_address = env::var("MONGODB_ADDRESS").expect("MONGODB_ADDRESS missing");
    let mongodb_port = env::var("MONGODB_PORT").expect("MONGODB_PORT missing");
    let database_name = env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME missing");
    let mongodb_user = env::var("MONGODB_USER").expect("MONGODB_USER missing");
    let mongodb_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD missing");

    let client = Client::with_uri_str(&format!(
        "mongodb://{}:{}@{}:{}/",
        mongodb_user, mongodb_password, mongodb_address, mongodb_port
    ))
    .await
    .expect("cannot create client");
    let db = client.database(&database_name);
    db
}
