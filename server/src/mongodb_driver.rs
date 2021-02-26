use mongodb::{Client, Database};
use std::env;
use std::ops::Deref;

#[derive(Clone)]
pub struct Connection(Database);

impl Connection {
    /// Create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
    pub async fn new() -> Self {
        let mongodb_address = env::var("MONGODB_ADDRESS").expect("MONGODB_ADDRESS missing");
        let mongodb_port = env::var("MONGODB_PORT").expect("MONGODB_PORT missing");
        let database_name =
            env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME missing");
        let mongodb_user = env::var("MONGODB_USER").expect("MONGODB_USER missing");
        let mongodb_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD missing");

        let client = Client::with_uri_str(&format!(
            "mongodb://{}:{}@{}:{}/",
            mongodb_user, mongodb_password, mongodb_address, mongodb_port
        ))
        .await
        .expect("cannot create client");
        let db = client.database(&database_name);
        Self(db)
    }
}

impl Deref for Connection {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
