-- Your SQL goes here
CREATE TABLE students (
  student_id INTEGER REFERENCES users (id),
  class_id INTEGER REFERENCES users (id)
);
