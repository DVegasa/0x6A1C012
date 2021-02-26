use std::{
    convert::{TryFrom, TryInto},
    fmt,
    num::ParseIntError,
    ops,
    str::FromStr,
};

use super::{Collection, Model};
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
// use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Copy, Clone)]
pub struct LessonTime {
    time: i32,
}

impl LessonTime {
    pub fn time_stamp(&self) -> String {
        format!("{}-{}", self, (*self + 45).to_string())
    }
}

impl ops::Add<i32> for LessonTime {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            time: self.time + rhs,
        }
    }
}

impl fmt::Display for LessonTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.time / 60, self.time % 60)
    }
}

impl FromStr for LessonTime {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split(":")
            .map(|v| v.parse::<i32>())
            .collect::<Vec<Result<i32, Self::Err>>>();
        let hours = vals.get(0).unwrap().to_owned()?;
        let minutes = vals.get(1).unwrap().to_owned()?;
        Ok(Self {
            time: hours * 60 + minutes,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Lesson {
    pub _id: ObjectId,
    pub teacher_id: ObjectId,
    pub subject_name: String,
    pub meeting_room: String,
    pub start_time: LessonTime,
}

impl Collection for Lesson {
    fn get_collection_name() -> &'static str {
        "lessons"
    }
}

impl Model for Lesson {
    fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
}

impl TryFrom<Document> for Lesson {
    type Error = bson::de::Error;
    fn try_from(document: Document) -> Result<Self, Self::Error> {
        bson::de::from_document(document)
    }
}

impl TryInto<Document> for Lesson {
    type Error = bson::ser::Error;
    fn try_into(self) -> Result<Document, Self::Error> {
        bson::ser::to_document(&self)
    }
}

#[cfg(test)]
mod test {
    use super::{Lesson, LessonTime};
    use mongodb::bson::{oid::ObjectId, Document};
    use pretty_assertions::assert_eq;
    use std::convert::{TryFrom, TryInto};

    #[test]
    fn try_deserialize_of_serialized_lesson() {
        let lesson = Lesson {
            _id: ObjectId::with_string("ffffffffffffffffffffffff").unwrap(),
            teacher_id: ObjectId::with_string("aaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
            start_time: LessonTime { time: 450 },
        };

        let doc: Document = lesson.clone().try_into().unwrap();
        let de_lesson: Lesson = doc.try_into().unwrap();
        assert_eq!(lesson, de_lesson);
    }
}
