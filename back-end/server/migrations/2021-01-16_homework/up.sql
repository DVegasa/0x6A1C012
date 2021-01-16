CREATE TABLE homework (
	subject_id INTEGER REFERENCES subject(id),
	teacher_id INTEGER REFERENCES teachers(teacher_id),
	homework_text VARCHAR(255),
	homework_file, ???
	created_at TIMESTAMP NOT NULL
);