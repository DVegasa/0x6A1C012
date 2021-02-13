use crate::models;
use crate::{errors::ServiceError, models::Pool};

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Deserialize)]
pub struct StudentScheduleRequestInfo {
    login: String,
    timestamp: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct WeekLessonTable([DayLessonTable; 6]);

#[derive(Serialize)]
pub struct DayLessonTable {
    lessons: Vec<Lesson>,
}

#[derive(Serialize)]
pub struct Lesson {
    subject_name: String,
    meeting_room: String,
    start_time: String,
}

impl Lesson {
    pub fn new(subject_name: String, meeting_room: String, start_time: i16) -> Self {
        let start_time = format!("{:02}:{:02}", start_time / 60, start_time % 60);
        Lesson {
            subject_name,
            meeting_room,
            start_time,
        }
    }
}

pub async fn get_schedule(
    query_data: web::Json<StudentScheduleRequestInfo>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(query_data.into_inner(), pool)).await;

    match res {
        Ok(_) => (),
        Err(_) => (),
    };

    Ok(HttpResponse::Ok().body(""))
}

// TODO: rewrite with join
/// Diesel query for db
fn query(
    query_data: StudentScheduleRequestInfo,
    pool: web::Data<Pool>,
) -> Result<WeekLessonTable, ServiceError> {
    use crate::schema::{class, class_student, lesson, subject, users, weekly_schedule};

    struct RawLessonInfo {
        subject_name: String,
        meeting_room: String,
        lesson_time: i16,
        lesson_week_day: i16,
        slot: i16,
    }

    // Take user by login
    let conn: &PgConnection = &pool.get().unwrap();

    let user = users::dsl::users
        .filter(users::dsl::login.eq(&query_data.login))
        .first::<models::User>(conn)?;

    let class_student = class_student::dsl::class_student
        .filter(class_student::dsl::student_id.eq(&user.id))
        .first::<models::ClassStudent>(conn)?;

    let class = class::dsl::class
        .filter(class::dsl::id.eq(class_student.class_id))
        .first::<models::Class>(conn)?;

    let weekly_schedeule = weekly_schedule::dsl::weekly_schedule
        .filter(weekly_schedule::dsl::id.eq(&class.weekly_schedule_id))
        .first::<models::WeeklySchedule>(conn)?;

    if let Some(lesson_ids) = weekly_schedeule.lesson_ids {
        let mut lessons = Vec::new();
        for lesson_id in lesson_ids {
            let lesson = lesson::dsl::lesson
                .filter(lesson::dsl::id.eq(lesson_id))
                .first::<models::Lesson>(conn)?;

            let subject = subject::dsl::subject
                .filter(subject::dsl::id.eq(lesson.subject_id))
                .first::<models::Subject>(conn)?;

            lessons.push(RawLessonInfo {
                subject_name: subject.subject_name,
                meeting_room: lesson.meeting_room,
                lesson_time: lesson.lesson_time,
                lesson_week_day: lesson.lesson_week_day,
                slot: lesson.slot,
            })
        }
    }

    Err(ServiceError::BadRequest(
        "Something went wrong Owo".to_owned(),
    ))
}
