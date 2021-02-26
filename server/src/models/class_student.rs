use std::convert::{TryFrom, TryInto};

use super::{Collection, Model};
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
// use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassStudent {
    /// Student
    pub _id: ObjectId,
    pub class: ObjectId,
}

impl Collection for ClassStudent {
    fn get_collection_name() -> &'static str {
        "class-student"
    }
}

impl Model for ClassStudent {
    fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
}

impl TryFrom<Document> for ClassStudent {
    type Error = bson::de::Error;
    fn try_from(document: Document) -> Result<Self, Self::Error> {
        bson::de::from_document(document)
    }
}

impl TryInto<Document> for ClassStudent {
    type Error = bson::ser::Error;
    fn try_into(self) -> Result<Document, Self::Error> {
        bson::ser::to_document(&self)
    }
}
