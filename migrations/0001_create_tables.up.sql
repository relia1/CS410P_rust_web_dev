CREATE TABLE tags (
	id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name TEXT NOT NULL
);

CREATE TABLE questions (
	id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL
);

/*
* Juntion table
*/
CREATE TABLE question_tags (
	question_id integer NOT NULL,
	tag_id integer NOT NULL,
	PRIMARY KEY (question_id, tag_id),
	FOREIGN KEY (question_id) REFERENCES questions(id),
	FOREIGN KEY (tag_id) REFERENCES tags(id)
);

