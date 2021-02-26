use {
    crate::{
        handlers::api::auth::logged_user_cookie::LoggedUserCookie,
        models::{
            self, class::Class, class_student::ClassStudent, lesson::Lesson, schedule::Schedule,
            user::User,
        },
        {errors::ServiceError, mongodb_driver::Connection},
    },
    actix_web::{web, HttpResponse},
    serde::Serialize,
    std::convert::TryInto,
};

#[derive(Serialize)]
pub struct WeekLessonTable(pub Vec<DayLessonTable>);

#[derive(Serialize)]
pub struct DayLessonTable(pub Vec<FormattedLesson>);

#[derive(Serialize)]
pub struct FormattedLesson {
    pub subject_name: String,
    pub meeting_room: String,
    pub start_time: String,
}

impl From<Lesson> for FormattedLesson {
    fn from(lesson: Lesson) -> Self {
        Self {
            subject_name: lesson.subject_name,
            meeting_room: lesson.meeting_room,
            start_time: lesson.start_time.to_string(),
        }
    }
}

pub async fn get_schedule(
    logged_user_cookie: LoggedUserCookie,
    connection: web::Data<Connection>,
) -> Result<HttpResponse, ServiceError> {
    let week_lesson_table = query(logged_user_cookie, connection).await?;

    Ok(HttpResponse::Ok().json(week_lesson_table))
}

// TODO: rewrite with join
/// Diesel query for db
async fn query(
    query_data: LoggedUserCookie,
    connection: web::Data<Connection>,
) -> Result<WeekLessonTable, ServiceError> {
    // TODO: think about this unwrap
    let user: User = models::get_by::<User>(&connection, "login", &query_data.login, None)
        .await?
        .unwrap_or(Err(ServiceError::InternalServerError)?)
        .try_into()
        .unwrap();

    let class_student: ClassStudent =
        models::get_by::<ClassStudent>(&connection, "_id", &user._id.to_string(), None)
            .await?
            .unwrap_or(Err(ServiceError::InternalServerError)?)
            .try_into()
            .unwrap();

    let class: Class =
        models::get_by::<Class>(&connection, "_id", &class_student.class.to_string(), None)
            .await?
            .unwrap_or(Err(ServiceError::InternalServerError)?)
            .try_into()
            .unwrap();

    let schedule: Schedule =
        models::get_by::<Schedule>(&connection, "_id", &class.schedule_id.to_string(), None)
            .await?
            .unwrap_or(Err(ServiceError::InternalServerError)?)
            .try_into()
            .unwrap();

    let mut week_schedule: WeekLessonTable = WeekLessonTable(Vec::new());
    for day_schedule in schedule.day_schedules.iter() {
        let mut day_lessons: DayLessonTable = DayLessonTable(Vec::new());
        for lesson_id in &day_schedule.lesson_ids {
            let lesson: Lesson =
                models::get_by::<Lesson>(&connection.clone(), "_id", &lesson_id.to_string(), None)
                    .await?
                    .unwrap_or(Err(ServiceError::InternalServerError)?)
                    .try_into()
                    .unwrap();
            let formatted_lesson: FormattedLesson = lesson.into();
            day_lessons.0.push(formatted_lesson);
        }
        week_schedule.0.push(day_lessons);
    }

    Ok(week_schedule)
}
