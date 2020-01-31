-- Your SQL goes here
alter table players
add team_id uuid;

alter table players
add constraint fk_player_team
foreign key (team_id) references teams(id);
