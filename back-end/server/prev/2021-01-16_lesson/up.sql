CREATE TABLE lesson (
	FOREIGN KEY (teacher_id) INTEGER REFERENCES teachers(teacher_id),
	metting_room VARCHAR(32),
	FOREIGN KEY (subject_name) VARCHAR REFERENCES subject(subject_name)
);
