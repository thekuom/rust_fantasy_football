-- This file should undo anything in `up.sql`
alter table players
drop foreign key fk_player_team;

alter table players
drop column team_id;
