use crate::models;
use crate::{errors::ServiceError, models::Pool};

use actix_rt::blocking::BlockingError;
use itertools::Itertools;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Deserialize)]
pub struct StudentScheduleRequestInfo {
    login: String,
}

#[derive(Serialize)]
pub struct WeekLessonTable(Vec<DayLessonTable>);

impl From<Vec<Vec<RawLessonInfo>>> for WeekLessonTable {
    fn from(week_lessons: Vec<Vec<RawLessonInfo>>) -> Self {
        <WeekLessonTable as From<Vec<DayLessonTable>>>::from(
            week_lessons
                .into_iter()
                .map(|day_lessons| DayLessonTable::from(day_lessons))
                .collect::<Vec<DayLessonTable>>(),
        )
    }
}

impl From<Vec<DayLessonTable>> for WeekLessonTable {
    fn from(week_lessons: Vec<DayLessonTable>) -> Self {
        Self(week_lessons)
    }
}

#[derive(Serialize)]
pub struct DayLessonTable {
    lessons: Vec<Lesson>,
}

impl From<Vec<RawLessonInfo>> for DayLessonTable {
    fn from(day_lessons: Vec<RawLessonInfo>) -> Self {
        let lessons = day_lessons
            .into_iter()
            .map(|raw_lesson| Lesson::from(raw_lesson))
            .collect();
        Self { lessons }
    }
}

#[derive(Serialize)]
pub struct Lesson {
    subject_name: String,
    meeting_room: String,
    start_time: String,
}

impl From<RawLessonInfo> for Lesson {
    fn from(raw_lesson_info: RawLessonInfo) -> Self {
        let start_time = format!(
            "{:02}:{:02}",
            raw_lesson_info.lesson_time / 60,
            raw_lesson_info.lesson_time % 60
        );
        Self {
            subject_name: raw_lesson_info.subject_name,
            meeting_room: raw_lesson_info.meeting_room,
            start_time,
        }
    }
}

pub struct RawLessonInfo {
    pub subject_name: String,
    pub meeting_room: String,
    pub lesson_time: i16,
    pub lesson_week_day: i16,
    pub slot: i16,
}

pub async fn get_schedule(
    query_data: web::Json<StudentScheduleRequestInfo>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(query_data.into_inner(), pool)).await;

    match res {
        Ok(week_lesson_table) => Ok(HttpResponse::Ok().json(week_lesson_table)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

// TODO: rewrite with join
/// Diesel query for db
fn query(
    query_data: StudentScheduleRequestInfo,
    pool: web::Data<Pool>,
) -> Result<WeekLessonTable, ServiceError> {
    use crate::schema::{class, class_student, lesson, subject, users, weekly_schedule};

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
        // Group lessons by week day
        let mut lessons_by_week_day/*: HashMap<i16/*week day*/, Vec<RawLessonInfo>>*/ = lessons
            .into_iter()
            .into_group_map_by(|raw_lesson_info| raw_lesson_info.lesson_week_day);

        // Group day lessons by slot
        for ref mut day_lessons in lessons_by_week_day.values_mut() {
            day_lessons.sort_by(|ref a, ref b| a.slot.cmp(&b.slot));
        }

        let mut lessons_by_week_day_matrix = Vec::new();
        for (_, day) in lessons_by_week_day.into_iter() {
            lessons_by_week_day_matrix.push(day);
        }

        return Ok(<WeekLessonTable as From<Vec<Vec<RawLessonInfo>>>>::from(
            lessons_by_week_day_matrix,
        ));
    }

    Err(ServiceError::BadRequest(
        "Something went wrong Owo".to_owned(),
    ))
}
