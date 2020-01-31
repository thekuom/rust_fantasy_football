-- Your SQL goes here
create table teams (
  id uuid primary key default gen_random_uuid(),
  display_name varchar not null,
  abbreviation varchar not null,
  created_at timestamp default now(),
  updated_at timestamp
)
