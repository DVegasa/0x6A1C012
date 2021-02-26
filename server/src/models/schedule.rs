use std::convert::{TryFrom, TryInto};

use super::{Collection, Model};
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DaySchedule {
    pub lesson_ids: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Schedule {
    pub _id: ObjectId,
    pub day_schedules: Vec<DaySchedule>,
}

impl Collection for Schedule {
    fn get_collection_name() -> &'static str {
        "schedules"
    }
}

impl Model for Schedule {
    fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
}

impl TryFrom<Document> for Schedule {
    type Error = bson::de::Error;
    fn try_from(document: Document) -> Result<Self, Self::Error> {
        bson::de::from_document(document)
    }
}

impl TryInto<Document> for Schedule {
    type Error = bson::ser::Error;
    fn try_into(self) -> Result<Document, Self::Error> {
        bson::ser::to_document(&self)
    }
}
