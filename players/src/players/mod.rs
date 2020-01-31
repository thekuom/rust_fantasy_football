use actix_web::{get, web, HttpRequest, Responder};
use diesel::prelude::*;
use serde::{Serialize};

mod models;
use models::{Player, Team};
use crate::establish_connection;

#[derive(Serialize)]
struct PlayerWithTeam {
    player: Player,
    team: Option<Team>,
}

#[get("/players")]
async fn get_players(_req: HttpRequest) -> impl Responder {
    use crate::schema::{players, teams};

    let connection = establish_connection();
    let players_with_teams = players::table
        .left_join(teams::table)
        .load::<(Player, Option<Team>)>(&connection)
        .expect("error with the query")
        .into_iter()
        .fold(Vec::new(), |mut result, (player, mut team)| {
            result.push(PlayerWithTeam {
                player,
                team: team.take(),
            });

            result
        });

    web::Json(players_with_teams)
}
