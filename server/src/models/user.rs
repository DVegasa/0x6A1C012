use std::str::FromStr;

use super::{Collection, Model};
use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId, Bson, Document};
use serde::{Deserialize, Serialize};
// use std::convert::TryFrom;

#[derive(PartialEq, Eq, PartialOrd, Ord, derive_more::Display, Debug, Serialize, Deserialize)]
pub enum UserRole {
    #[display(fmt = "A")]
    Admin,
    #[display(fmt = "T")]
    Teacher,
    #[display(fmt = "S")]
    Student,
    #[display(fmt = "P")]
    Parent,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first_char) = s.chars().next() {
            return Ok(match first_char {
                'A' => Self::Admin,
                'T' => Self::Teacher,
                'S' => Self::Student,
                'P' => Self::Parent,
                _ => return Err(()),
            });
        };
        Err(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub login: String,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub pswd_hash: String,
    pub role: UserRole,
    pub created_at: String, // TODO: use `chrono::DateTime<Utc>` instead of string
}

impl Collection for User {
    fn get_collection_name() -> &'static str {
        "users"
    }
}
impl Model for User {
    fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            created_at: Utc::now().to_string(),
            ..Default::default()
        }
    }
}

impl From<Document> for User {
    fn from(document: Document) -> Self {
        let mut user_constructor = Self::default();
        if let Ok(_id) = document.get_object_id("_id") {
            // use unwrap because almost a 100% this will be normal string
            // TODO: implement error handling with using `super::ConvertionError`
            user_constructor._id = _id.to_owned();
        }
        if let Some(&Bson::String(ref _login)) = document.get("login") {
            user_constructor.login = _login.to_owned();
        }
        if let Some(&Bson::String(ref _firstname)) = document.get("firstname") {
            user_constructor.firstname = _firstname.to_owned();
        }
        if let Some(&Bson::String(ref _middlename)) = document.get("middlename") {
            user_constructor.middlename = _middlename.to_owned();
        }
        if let Some(&Bson::String(ref _lastname)) = document.get("lastname") {
            user_constructor.lastname = _lastname.to_owned();
        }
        if let Some(&Bson::String(ref _pswd_hash)) = document.get("pswd_hash") {
            user_constructor.pswd_hash = _pswd_hash.to_owned();
        }
        if let Some(&Bson::String(ref _created_at)) = document.get("created_at") {
            user_constructor.created_at = _created_at.to_owned();
        }
        user_constructor
    }
}
impl Into<Document> for User {
    fn into(self) -> Document {
        mongodb::bson::doc! {
            "_id": self._id,
            "login": self.login,
            "firstname": self.firstname,
            "middlename": self.middlename,
            "lastname": self.lastname,
            "pswd_hash": self.pswd_hash,
            "created_at": self.created_at,
        }
    }
}
