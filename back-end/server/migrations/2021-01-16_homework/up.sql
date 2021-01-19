CREATE TABLE homework (
	FOREIGN KEY (subject_id) INTEGER REFERENCES subject(id),
	FOREIGN KEY (teacher_id) INTEGER REFERENCES teachers(teacher_id),
	homework_text VARCHAR(255),
	homework_file, ???
	created_at TIMESTAMP NOT NULL
);
