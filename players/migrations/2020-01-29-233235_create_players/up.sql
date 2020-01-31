-- Your SQL goes here
create table players (
  id uuid primary key default gen_random_uuid(),
  first_name varchar not null,
  last_name varchar not null,
  created_at timestamp default now(),
  updated_at timestamp
)
