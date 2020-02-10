/// This file will hold our player related routes

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind as DbError;
use uuid::Uuid;

// AppData is defined in src/lib.rs, which is our entrypoint
use crate::AppData;
use crate::common::JsonError;

// Re-export models. Right now this is only for the tests. Ideally this could
// remain encapsulated within the module
pub mod models;
use models::{CreatePlayerForm, Player, PlayerWithTeam, Team, UpdatePlayerForm};

/// Gets all the players and their team from the database
///
/// # Panics
///
/// Panics when it fails to get a database connection
///
/// Panics when it fails to query the database
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

/// Creates a player
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn create_player(
    data: web::Data<AppData>,
    player: web::Json<CreatePlayerForm>
) -> impl Responder {
    use crate::schema::players;

    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let player = player.into_inner();
    let team_not_found_err = format!("Team {} not found", &player.team_id.map(|id| id.to_string()).unwrap_or_else(|| "".to_string()));

    let result = diesel::insert_into(players::table)
        .values(player)
        .get_result::<Player>(&connection);

    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(err) => match err {
            DieselError::DatabaseError(DbError::ForeignKeyViolation, _) => 
                HttpResponse::BadRequest().json(JsonError::<bool> {
                    message: team_not_found_err,
                    data: None,
                }),
            _ => HttpResponse::InternalServerError().json(JsonError {
                message: "Something went wrong".to_string(),
                data: Some(err.to_string()),
            }),
        },
    }
}

/// Updates a player
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn update_player(
    data: web::Data<AppData>,
    path: web::Path<Uuid>,
    player: web::Json<UpdatePlayerForm>
) -> impl Responder {
    use crate::schema::players;

    println!("update data: {:?}", player);

    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let player = player.into_inner();
    let id = path.into_inner();

    let result = diesel::update(players::table.find(&id)).set(&player).get_result::<Player>(&connection);

    match result {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(err) => HttpResponse::InternalServerError().json(JsonError {
            message: "Something went wrong".to_string(),
            data: Some(err.to_string()),
        }),
    }
}
