use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind as DbError;
use diesel::result::Error as DieselError;
use uuid::Uuid;

use crate::AppData;
use crate::common::JsonError;
use crate::schema::teams;

pub mod models;
use models::{CreateTeamForm, Team, UpdateTeamForm};

/// Gets all the teams
///
/// # Returns
///
/// 200 is returned and sends an array of [Team](./models/struct.Team.html)
///
/// # Panics
///
/// Panics when it fails to get a database connection
///
/// Panics when it fails to query the database
pub async fn get_teams(
    data: web::Data<AppData>,
    _req: HttpRequest
) -> impl Responder {
    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let result = teams::table.load::<Team>(&connection).expect("error with the query");

    println!("result: {:?}", result);

    HttpResponse::Ok().json(result)
}


/// Fetches a team
///
/// # Returns
///
/// 200 is returned if the team is found and sends a [Team](./models/struct.Team.html) back
///
/// 404 is returned when the team is not found by the given id
///
/// 500 is returned when there is any other database error
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn get_team(
    data: web::Data<AppData>,
    path: web::Path<Uuid>,
    _req: HttpRequest
) -> impl Responder {
    let id = path.into_inner();

    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let result = teams::table.find(id).first::<Team>(&connection);

    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => match err {
            DieselError::NotFound => HttpResponse::NotFound().json(JsonError {
                message: "Team not found".to_string(),
                data: Some(id),
            }),
            _ => HttpResponse::InternalServerError().json(JsonError {
                message: "Something went wrong".to_string(),
                data: Some(err.to_string()),
            }),
        }
    }
}

/// Creates a team
///
/// # Returns
///
/// 200 is returned when the creation is successful and sends the created
///     [Team](./models/struct.Team.html)
///
/// 500 is returned when there is any other database error
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn create_team(
    data: web::Data<AppData>,
    team: web::Json<CreateTeamForm>
) -> impl Responder {
    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let team = team.into_inner();

    let result = diesel::insert_into(teams::table)
        .values(team)
        .get_result::<Team>(&connection);

    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => HttpResponse::InternalServerError().json(JsonError {
            message: "Something went wrong".to_string(),
            data: Some(err.to_string()),
        }),
    }
}

/// Updates a team
///
/// # Returns
///
/// 200 is returned when the update was successful and sends the updated
///     [Team](./models/struct.Team.html)
///
/// 500 is returned when there is any other database error
///
/// # Panics
///
/// Panics when it fails to get a database connection
pub async fn update_team(
    data: web::Data<AppData>,
    path: web::Path<Uuid>,
    team: web::Json<UpdateTeamForm>
) -> impl Responder {
    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let team = team.into_inner();
    let id = path.into_inner();

    let result = diesel::update(teams::table.find(&id)).set(&team).get_result::<Team>(&connection);

    match result {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => match err {
            DieselError::NotFound => HttpResponse::NotFound().json(JsonError {
                message: "Team not found".to_string(),
                data: Some(id),
            }),
            _ => HttpResponse::InternalServerError().json(JsonError {
                message: "Something went wrong".to_string(),
                data: Some(err.to_string()),
            }),
        },
    }
}

/// Deletes a team
///
/// # Returns
///
/// 204 is returned when the delete was successful or the team does not exist
///
/// 400 is returned if the team still has players
///
/// 500 is returned when there is any other database error
///
/// # Panics
///
/// Panics when it fais to get a database connection
pub async fn delete_team(
    data: web::Data<AppData>,
    path: web::Path<Uuid>,
    _req: HttpRequest
) -> impl Responder {
    let connection = data.db_pool.get().expect("Could not get db connection from pool");
    let id = path.into_inner();

    match diesel::delete(teams::table.find(&id)).execute(&connection) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => match err {
            DieselError::DatabaseError(DbError::ForeignKeyViolation, _) =>
                HttpResponse::BadRequest().json(JsonError::<bool> {
                    message: "Cannot delete team: players still exist in the team".to_string(),
                    data: None,
                }),
            _ => HttpResponse::InternalServerError().json(JsonError {
                message: "Something went wrong".to_string(),
                data: Some(err.to_string()),
            })
        }
    }
}
