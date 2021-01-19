create table attendance (
    FOREIGN KEY (subject_id) INTEGER REFERENCES subject(id),
    FOREIGN KEY (teacher_id) INTEGER REFERENCES teachers(teacher_id),
    FOREIGN KEY (student_id) INTEGER REFERENCES students (student_id),
    state BOOLEAN,
    created_at TIMESTAMP NOT NULL
);
