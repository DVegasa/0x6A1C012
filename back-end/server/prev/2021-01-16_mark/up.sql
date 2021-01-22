CREATE TABLE mark (
	FOREIGN KEY (subject_id) INTEGER REFERENCES subject(id),
	FOREIGN KEY (teacher_id) INTEGER REFERENCES teachers(teacher_id),
	FOREIGN KEY (student_id) INTEGER REFERENCES students (student_id),
	mark VARCHAR(2),
	coeffiecient VARCHAR(2),
	created_at TIMESTAMP NOT NULL
);
