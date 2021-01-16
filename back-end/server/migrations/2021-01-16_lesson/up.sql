CREATE TABLE lesson (
	teacher_id INTEGER REFERENCES teachers(teacher_id),
	metting_room VARCHAR(32),
	subject_name VARCHAR REFERENCES subject(subject_name)
);