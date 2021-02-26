use super::{Collection, Model};
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Class {
    pub _id: ObjectId,
    pub class_teacher_id: ObjectId,
    pub schedule_id: ObjectId,
    pub study_year: i32,
    pub letter: char,
}

impl Collection for Class {
    fn get_collection_name() -> &'static str {
        "classes"
    }
}

impl Model for Class {
    fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
}

impl TryFrom<Document> for Class {
    type Error = bson::de::Error;
    fn try_from(document: Document) -> Result<Self, Self::Error> {
        bson::de::from_document(document)
    }
}

impl TryInto<Document> for Class {
    type Error = bson::ser::Error;
    fn try_into(self) -> Result<Document, Self::Error> {
        bson::ser::to_document(&self)
    }
}
