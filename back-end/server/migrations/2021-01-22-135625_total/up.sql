-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    login VARCHAR(32) NOT NULL,
    firstname VARCHAR(32) NOT NULL,
    middlename VARCHAR(32) NOT NULL,
    lastname VARCHAR(32) NOT NULL,
    pswd_hash VARCHAR(128) NOT NULL,
	role SMALLINT,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE subject ( 
    id SERIAL PRIMARY KEY,
    subject_name TEXT,
    teacher_id INTEGER,
	FOREIGN KEY (teacher_id) REFERENCES users(id)
);

CREATE TABLE lesson (
    id SERIAL PRIMARY KEY,
    teacher_id INTEGER,
    meeting_room TEXT,
    subject_id INTEGER,
	slot SMALLINT,
    lesson_date DATE NOT NULL,
	FOREIGN KEY (teacher_id) REFERENCES users(id),
	FOREIGN KEY (subject_id) REFERENCES subject(id)
);

CREATE TABLE attendance (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER,
    student_id INTEGER,
    state VARCHAR(2),
	FOREIGN KEY (lesson_id) REFERENCES lesson(id),
	FOREIGN KEY (student_id) REFERENCES lesson(id)
);

CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    classroom_teacher_id INTEGER,
    year_of_study SMALLINT,
    letter VARCHAR(1),
    FOREIGN KEY (classroom_teacher_id) REFERENCES users(id)
);

CREATE TABLE class_student (
    id SERIAL PRIMARY KEY,
    student_id INTEGER,
	class_id INTEGER,
    FOREIGN KEY (student_id) REFERENCES users(id),
    FOREIGN KEY (class_id) REFERENCES class(id)
);

CREATE TABLE homework (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER,
    homework_text TEXT,
    deadline TIMESTAMP NOT NULL,
    FOREIGN KEY (lesson_id) REFERENCES lesson(id)
);

CREATE TABLE mark (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER,
    teacher_id INTEGER,
    student_id INTEGER,
    mark_value VARCHAR(2),
    coeffiecient REAL,
    FOREIGN KEY (lesson_id) REFERENCES lesson(id),
    FOREIGN KEY (teacher_id) REFERENCES users(id),
    FOREIGN KEY (student_id) REFERENCES users(id)
);

CREATE TABLE observation (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER,
    teacher_id INTEGER,
    student_id INTEGER,
    description TEXT,
    FOREIGN KEY (lesson_id) REFERENCES lesson(id),
    FOREIGN KEY (teacher_id) REFERENCES users(id),
    FOREIGN KEY (student_id) REFERENCES users(id)
);

