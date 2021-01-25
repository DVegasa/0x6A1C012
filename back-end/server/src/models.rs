use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub login: String,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
    pub pswd_hash: String,
    pub role: i16,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "subject"]
pub struct Subject {
    pub id: i32,
    pub subject_name: String,
    pub teacher_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "lesson"]
pub struct Lesson {
    pub id: i32,
    pub teacher_id: i32,
    pub meeting_room: String,
    pub subject_id: i32,
    pub slot: i16,
    pub lesson_date: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "attendance"]
pub struct Attendance {
    pub id: i32,
    pub lesson_id: i32,
    pub student_id: i32,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "class"]
pub struct Class {
    pub id: i32,
    pub classroom_teacher_id: i32,
    pub year_of_study: i16,
    pub letter: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "class_student"]
pub struct ClassStudent {
    pub id: i32,
    pub student_id: i32,
    pub class_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "homework"]
pub struct Homework {
    pub id: i32,
    pub lesson_id: i32,
    pub homework_text: String,
    pub deadline: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "mark"]
pub struct Mark {
    pub id: i32,
    pub lesson_id: i32,
    pub teacher_id: i32,
    pub student_id: i32,
    pub mark_value: String,
    pub coeffiecient: f32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "observation"]
pub struct Observation {
    pub id: i32,
    pub lesson_id: i32,
    pub teacher_id: i32,
    pub student_id: i32,
    pub description: String,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(login: S, pwd: T) -> Self {
        User {
            login: login.into(),
            pswd_hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub login: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { login: user.login }
    }
}
