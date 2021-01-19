-- Your SQL goes here
CREATE TABLE parents (
    FOREIGN KEY (parent_id) INTEGER REFERENCES users (id),
    FOREIGN KEY (chilldrens_id) INTEGER[] REFERENCES users (id),
);
