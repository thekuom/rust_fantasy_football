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
/// # Returns
///
/// 200 is returned and sends an array of [PlayerWithTeam](./models/struct.PlayerWithTeam.html)
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

/// Fetches a player
///
/// # Returns
///
/// 200 is returned if the player is found and sends a
///     [PlayerWithTeam](./models/struct.PlayerWithTeam.html)
///
/// 404 is returned when the player is not found by the given id
///
/// 500 is returned when there is any other database error
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn get_player(
    data: web::Data<AppData>,
    path: web::Path<Uuid>,
    _req: HttpRequest
) -> impl Responder {
    use crate::schema::{players, teams};

    let id = path.into_inner();

    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let result = players::table
        .find(id)
        .left_join(teams::table)
        .first::<(Player, Option<Team>)>(&connection);

    match result {
        Ok((player, team)) => {
            HttpResponse::Ok().json(PlayerWithTeam {
                player,
                team,
            })
        },
        Err(err) => match err {
            DieselError::NotFound => HttpResponse::NotFound().json(JsonError::<bool> {
                message: "Player not found".to_string(),
                data: None,
            }),
            _ => HttpResponse::InternalServerError().json(JsonError {
                message: "Something went wrong".to_string(),
                data: Some(err.to_string()),
            }),
        },
    }
}

/// Creates a player
///
/// # Returns
///
/// 200 is returned when the creation is successful and sends the created
///     [Player](./models/struct.Player.html)
///
/// 400 is returned when there is a foreign key violation i.e. the team does not exist
///
/// 500 is returned when there is any other database error
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
/// # Returns
///
/// 200 is returned when the update was successful and sends the updated
///     [Player](./models/struct.Player.html)
///
/// 500 is returned when there is any other database error
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
