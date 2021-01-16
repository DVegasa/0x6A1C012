create table observation (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    student_id INTEGER REFERENCES students (student_id),
    description TEXT,
    created_at TIMESTAMP NOT NULL
);