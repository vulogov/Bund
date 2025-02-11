drop table if exists sample;
create table sample (
	ID	INTEGER,
	Name	TEXT,
	Age	INTEGER,
	DateOfBirth TEXT,
	Score	FLOAT,
	Active  BOOL,
	Description TEXT,
	JoinDate  TEXT
);
.mode csv
.import sample.csv sample
