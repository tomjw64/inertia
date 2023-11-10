-- Add migration script here
create table if not exists boards (
  id integer primary key not null,
  difficulty integer not null,
  solution text not null,
  board blob not null
);
create index difficulty_level on boards(difficulty);
