-- Your SQL goes here
CREATE TABLE students (
  FOREIGN KEY (student_id) INTEGER REFERENCES users (id),
  FOREIGN KEY (class_id) INTEGER REFERENCES class (id)
);
