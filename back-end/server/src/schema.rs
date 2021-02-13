table! {
    attendance (id) {
        id -> Int4,
        lesson_id -> Nullable<Int4>,
        student_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
    }
}

table! {
    class (id) {
        id -> Int4,
        classroom_teacher_id -> Int4,
        weekly_schedule_id -> Int4,
        year_of_study -> Int2,
        letter -> Varchar,
    }
}

table! {
    class_student (id) {
        id -> Int4,
        student_id -> Int4,
        class_id -> Int4,
    }
}

table! {
    homework (id) {
        id -> Int4,
        lesson_id -> Nullable<Int4>,
        homework_text -> Nullable<Text>,
        deadline -> Timestamp,
    }
}

table! {
    lesson (id) {
        id -> Int4,
        teacher_id -> Int4,
        meeting_room -> Text,
        subject_id -> Int4,
        slot -> Int2,
        lesson_time -> Int2,
        lesson_week_day -> Int2,
    }
}

table! {
    mark (id) {
        id -> Int4,
        lesson_id -> Nullable<Int4>,
        teacher_id -> Nullable<Int4>,
        student_id -> Nullable<Int4>,
        mark_value -> Nullable<Varchar>,
        coeffiecient -> Nullable<Float4>,
    }
}

table! {
    observation (id) {
        id -> Int4,
        lesson_id -> Nullable<Int4>,
        teacher_id -> Nullable<Int4>,
        student_id -> Nullable<Int4>,
        description -> Nullable<Text>,
    }
}

table! {
    subject (id) {
        id -> Int4,
        subject_name -> Nullable<Text>,
        teacher_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        firstname -> Varchar,
        middlename -> Varchar,
        lastname -> Varchar,
        pswd_hash -> Varchar,
        role -> Nullable<Int2>,
        created_at -> Timestamp,
    }
}

table! {
    weekly_schedule (id) {
        id -> Int4,
        lesson_ids -> Nullable<Array<Int4>>,
    }
}

joinable!(class -> users (classroom_teacher_id));
joinable!(class -> weekly_schedule (weekly_schedule_id));
joinable!(class_student -> class (class_id));
joinable!(class_student -> users (student_id));
joinable!(homework -> lesson (lesson_id));
joinable!(lesson -> subject (subject_id));
joinable!(lesson -> users (teacher_id));
joinable!(mark -> lesson (lesson_id));
joinable!(observation -> lesson (lesson_id));
joinable!(subject -> users (teacher_id));

allow_tables_to_appear_in_same_query!(
    attendance,
    class,
    class_student,
    homework,
    lesson,
    mark,
    observation,
    subject,
    users,
    weekly_schedule,
);
