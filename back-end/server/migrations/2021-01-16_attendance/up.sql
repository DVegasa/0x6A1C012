create table attendance (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    student_id INTEGER REFERENCES students (student_id),
    state BOOLEAN,
    created_at TIMESTAMP NOT NULL
);