CREATE TABLE answers (
	id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	answer TEXT NOT NULL,
	question_id INTEGER,
	FOREIGN KEY (question_id) REFERENCES questions(id)
);
