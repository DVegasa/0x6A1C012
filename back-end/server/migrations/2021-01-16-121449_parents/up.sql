-- Your SQL goes here
CREATE TABLE parents (
    parent_id INTEGER REFERENCES users (id),
    chilldrens_id INTEGER[] REFERENCES users (id),
);
