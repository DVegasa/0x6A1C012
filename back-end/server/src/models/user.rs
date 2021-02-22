use crate::mongodb_driver::Conn;
use mongodb::{bson::doc, error::Error};
use serde_derive::{Deserialize, Serialize};

const COLLECTION: &'static str = "users";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: Option<bson::oid::ObjectId>,
    pub login: Option<String>,
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub lastname: Option<String>,
    pub pswd_hash: Option<String>,
    pub created_at: Option<String>,
}

pub async fn delete_all(connection: &Conn) -> Result<(), Error> {
    connection.collection(COLLECTION).drop(None).await
}
