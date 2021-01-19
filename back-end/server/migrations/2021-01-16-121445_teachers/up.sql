-- Your SQL goes here
CREATE TABLE teachers (
    FOREIGN KEY (teacher_id) INTEGER REFERENCES users (id),
);
