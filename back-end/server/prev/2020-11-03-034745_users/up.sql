-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  login VARCHAR(32) NOT NULL,
  name VARCHAR(32) NOT NULL,
  surname VARCHAR(32) NOT NULL,
  lastname VARCHAR(32) NOT NULL,
  pswd_hash VARCHAR(128) NOT NULL, --argon hash
  role INTEGER,
  created_at TIMESTAMP NOT NULL
);
