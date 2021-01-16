CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES teachers(teacher_id)
    class_name ???
    student_id ???
);
