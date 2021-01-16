-- Your SQL goes here
CREATE TABLE teachers (
    teacher_id INTEGER REFERENCES users (id),
);
