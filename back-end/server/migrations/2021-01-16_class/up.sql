CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    FOREIGN KEY (teacher_id) INTEGER REFERENCES teachers(teacher_id)
    class_name ???
    student_id ???
);
