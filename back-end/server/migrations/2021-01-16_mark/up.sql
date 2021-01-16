CREATE TABLE mark (
	subject_id INTEGER REFERENCES subject(id),
	teacher_id INTEGER REFERENCES teachers(teacher_id),
	student_id INTEGER REFERENCES students (student_id),
	mark VARCHAR(2),
	coeffiecient VARCHAR(2),
	created_at TIMESTAMP NOT NULL
);