-- Your SQL goes here

CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  hash VARCHAR(128) NOT NULL, --argon hash
  created_at TIMESTAMP NOT NULL
);

