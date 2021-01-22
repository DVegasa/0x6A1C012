-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  login VARCHAR(32) NOT NULL,
  name VARCHAR(32) NOT NULL,
  surname VARCHAR(32) NOT NULL,
  lastname VARCHAR(32) NOT NULL,
  pswd_hash VARCHAR(128) NOT NULL,
  role INTEGER,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE students (
  student_id INTEGER REFERENCES users(id),
  class_id INTEGER REFERENCES users(id)
);

CREATE TABLE teachers (
    teacher_id INTEGER REFERENCES users(id)
);

CREATE TABLE parents (
    parent_id INTEGER REFERENCES users(id),
    childrens_id INTEGER[] REFERENCES users(id)
);

CREATE TABLE attendance (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    student_id INTEGER REFERENCES students(student_id),
    state BOOLEAN,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE class (
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    class_name VARCHAR(4),
    student_id INTEGER[] REFERENCES users(id)
);

CREATE TABLE homework (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    homework_text VARCHAR(255),
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE lesson (
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    metting_room VARCHAR(32),
    subject_name VARCHAR REFERENCES subject(subject_name)
);

CREATE TABLE mark (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    student_id INTEGER REFERENCES students(student_id),
    mark VARCHAR(2),
    coeffiecient VARCHAR(2),
    created_at TIMESTAMP NOT NULL
);

create table observation (
    subject_id INTEGER REFERENCES subject(id),
    teacher_id INTEGER REFERENCES teachers(teacher_id),
    student_id INTEGER REFERENCES students(student_id),
    description TEXT,
    created_at TIMESTAMP NOT NULL
);
