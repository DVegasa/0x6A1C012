pub mod class;
pub mod class_student;
pub mod lesson;
pub mod schedule;
pub mod user;

use crate::mongodb_driver::Connection;
// use derive_more::{Display, Error, From};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error,
    options::{DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

pub trait Collection {
    fn get_collection_name() -> &'static str;
}

pub trait Model {
    fn get_id(&self) -> ObjectId;
}

/// Create new  document in collection
pub async fn create<M>(
    connection: &Connection,
    self_struct: M,
    options: Option<InsertOneOptions>,
) -> Result<InsertOneResult, Error>
where
    M: Collection + Into<Document>,
{
    connection
        .collection(M::get_collection_name())
        .insert_one(self_struct.into(), options)
        .await
}

/// Get all documents in collection
pub async fn get<M>(connection: &Connection, options: Option<FindOptions>) -> Result<Vec<M>, Error>
where
    M: Collection + From<Document>,
{
    let mut cursor = connection
        .collection(M::get_collection_name())
        .find(None, options)
        .await?;
    let mut data: Vec<M> = Vec::new();

    while let Some(item) = cursor.next().await {
        if let Ok(result) = item {
            data.push(result.into())
        }
    }
    Ok(data)
}

/// Get one document by field name and value
pub async fn get_by<M>(
    connection: &Connection,
    requested_field: &str,
    requested_value: &str,
    options: Option<FindOneOptions>,
) -> Result<Option<Document>, Error>
where
    M: Collection,
{
    connection
        .collection(M::get_collection_name())
        .find_one(doc! { requested_field: requested_value }, options)
        .await
}

/// Update document in database by _id field
pub async fn update<M>(
    connection: &Connection,
    struct_new_version: M,
    options: Option<UpdateOptions>,
) -> Result<UpdateResult, Error>
where
    M: Collection + Model + Into<Document>,
{
    connection
        .collection(M::get_collection_name())
        .update_one(
            doc! { "_id": struct_new_version.get_id().to_hex() },
            <M as Into<Document>>::into(struct_new_version),
            options,
        )
        .await
}

/// Delete document in collection
pub async fn delete<M>(
    connection: &Connection,
    _id: &str,
    options: Option<DeleteOptions>,
) -> Result<DeleteResult, Error>
where
    M: Collection,
{
    connection
        .collection(M::get_collection_name())
        .delete_one(doc! { "_id": _id }, options)
        .await
}

// TODO: rewrite `impl TryFrom<Document> for User` with normal error handling
// #[derive(Error, From, Display)]
// pub enum ConvertionError {
//     #[display(fmt = "Field {} not found", _0)]
//     FieldNotFound(&'static str),
//     ObjectIdConvertionError(mongodb::bson::oid::Error)
// }

// // use super::schema::*;
// use crate::schema::*;
// use diesel::{r2d2::ConnectionManager, PgConnection};
// use serde::{Deserialize, Serialize};

// // type alias to use in multiple places
// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[diesel(table_name = "users")]
// pub struct User {
//     pub id: i32,
//     pub login: String,
//     pub firstname: String,
//     pub middlename: String,
//     pub lastname: String,
//     pub pswd_hash: String,
//     pub role: Option<i16>,
//     pub created_at: chrono::NaiveDateTime,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "subject")]
// pub struct Subject {
//     pub id: i32,
//     pub subject_name: String,
//     pub teacher_id: i32,
// }

// // TODO: Insertable version of this struct
// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "lesson")]
// pub struct Lesson {
//     pub id: i32,
//     pub teacher_id: i32,
//     pub meeting_room: String,
//     pub subject_id: i32,
//     pub slot: i16,
//     pub lesson_time: i16,
//     pub lesson_week_day: i16,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "attendance")]
// pub struct Attendance {
//     pub id: i32,
//     pub lesson_id: i32,
//     pub student_id: i32,
//     pub state: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "weekly_schedule")]
// pub struct WeeklySchedule {
//     pub id: i32,
//     pub lesson_ids: Option<Vec<i32>>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "class")]
// pub struct Class {
//     pub id: i32,
//     pub classroom_teacher_id: i32,
//     pub weekly_schedule_id: i32,
//     pub year_of_study: i16,
//     pub letter: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "class_student")]
// pub struct ClassStudent {
//     pub id: i32,
//     pub student_id: i32,
//     pub class_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "homework")]
// pub struct Homework {
//     pub id: i32,
//     pub lesson_id: i32,
//     pub homework_text: String,
//     pub deadline: chrono::NaiveDateTime,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "mark")]
// pub struct Mark {
//     pub id: i32,
//     pub lesson_id: i32,
//     pub teacher_id: i32,
//     pub student_id: i32,
//     pub mark_value: String,
//     pub coeffiecient: f32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = "observation")]
// pub struct Observation {
//     pub id: i32,
//     pub lesson_id: i32,
//     pub teacher_id: i32,
//     pub student_id: i32,
//     pub description: String,
// }

// // impl User {
// //     pub fn from_details<S: Into<String>, T: Into<String>>(login: S, pwd: T) -> Self {
// //         User {
// //             login: login.into(),
// //             pswd_hash: pwd.into(),
// //             created_at: chrono::Local::now().naive_local(),
// //         }
// //     }
// // }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SlimUser {
//     pub login: String,
// }

// impl From<User> for SlimUser {
//     fn from(user: User) -> Self {
//         SlimUser { login: user.login }
//     }
// }
