CREATE TABLE subject ( 
	id SERIAL PRIMARY KEY
	subject_name VARCHAR(32) NOT NULL,
	subject_place VARCHAR(32) NOT NULL,
	FOREIGN KEY (teacher_id) INTEGER REFERENCES users(id)
);
