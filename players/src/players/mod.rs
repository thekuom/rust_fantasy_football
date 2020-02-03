use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AppData;

pub mod models;
use models::{Player, Team};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerWithTeam {
    pub player: Player,
    pub team: Option<Team>,
}

pub async fn get_players(
    data: web::Data<AppData>,
    _req: HttpRequest
) -> impl Responder {
    use crate::schema::{players, teams};

    let connection = data.db_pool.get().expect("Could not get db connection from pool");
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

    HttpResponse::Ok().json(players_with_teams)
}
