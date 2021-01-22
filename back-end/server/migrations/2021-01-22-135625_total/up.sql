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
    teacher_id INTEGER REFERENCES users(id)
);

CREATE TABLE lesson (
    id SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES users(id),
    meeting_room TEXT,
    subject_id INTEGER REFERENCES subject(id),
	slot SMALLINT,
    lesson_date DATE NOT NULL
);

CREATE TABLE attendance (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER REFERENCES lesson(id),
    student_id INTEGER REFERENCES users(id),
    state VARCHAR(2)
);

CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    classroom_teacher_id INTEGER,
    year_of_study SMALLINT,
    letter VARCHAR(1),
    student_id INTEGER[] REFERENCES users(id),
    FOREIGN KEY (classroom_teacher_id) REFERENCES users(id)
);

CREATE TABLE homework (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER REFERENCES lesson(id),
    homework_text TEXT,
    deadline TIMESTAMP NOT NULL
);

CREATE TABLE mark (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER REFERENCES lesson(id),
    teacher_id INTEGER REFERENCES users(id),
    student_id INTEGER REFERENCES users(id),
    mark VARCHAR(2),
    coeffiecient REAL
);

CREATE TABLE observation (
    id SERIAL PRIMARY KEY,
    lesson_id INTEGER REFERENCES lesson(id),
    teacher_id INTEGER REFERENCES users(id),
    student_id INTEGER REFERENCES users(id),
    description TEXT
);

