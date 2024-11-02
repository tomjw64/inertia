-- Add migration script here
create table if not exists solved_positions (
  id integer primary key not null,
  difficulty integer not null,
  solution text not null,
  position blob not null,
  difficulty_ordinal integer not null
);
create index difficulty_level on solved_positions(difficulty, difficulty_ordinal);
