use serde_derive::{Deserialize, Serialize};

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
