/// The models needed for the players APIs

// Deserialize and Serialize help translate to and from JSON
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::common::JsonError;
use crate::schema::{players, teams};

/// Player model. Matches the database.
#[derive(Associations, Debug, Deserialize, Identifiable, Insertable, Serialize, Queryable)]
#[belongs_to(Team)]
#[table_name = "players"]
pub struct Player {
    pub id: Uuid, 
    pub first_name: String,
    pub last_name: String,
    #[serde(skip)]
    pub created_at: Option<SystemTime>,
    #[serde(skip)]
    pub updated_at: Option<SystemTime>,
    #[serde(skip_serializing)]
    pub team_id: Option<Uuid>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.id == other.id
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "players"]
pub struct CreatePlayerForm {
    pub first_name: String,
    pub last_name: String,
    pub team_id: Option<Uuid>,
}

impl CreatePlayerForm {
    pub fn handle_deserialize(cfg: web::JsonConfig) -> web::JsonConfig {
        cfg.error_handler(|err, _req| {
            let err_message = format!("{}", &err);
            error::InternalError::from_response(
                err, HttpResponse::BadRequest().json(JsonError::<bool> {
                    message: err_message,
                    data: None,
                })).into()
        })
    }
}

#[changeset_options(treat_none_as_null="true")]
#[derive(AsChangeset, Deserialize)]
#[table_name = "players"]
pub struct UpdatePlayerForm {
    pub first_name: String,
    pub last_name: String,
    pub team_id: Option<Uuid>,
}

impl UpdatePlayerForm {
    pub fn handle_deserialize(cfg: web::JsonConfig) -> web::JsonConfig {
        cfg.error_handler(|err, _req| {
            let err_message = format!("{}", &err);
            error::InternalError::from_response(
                err, HttpResponse::BadRequest().json(JsonError::<bool> {
                    message: err_message,
                    data: None,
                })).into()
        })
    }
}

/// Team model. Represents a team a player can be on
#[derive(Identifiable, Debug, Deserialize, Serialize, Queryable)]
#[table_name = "teams"]
pub struct Team {
    pub id: Uuid,
    pub display_name: String,
    pub abbreviation: String,
    #[serde(skip)]
    pub created_at: Option<SystemTime>,
    #[serde(skip)]
    pub updated_at: Option<SystemTime>,
}

impl PartialEq for Team {
    fn eq(&self, other: &Team) -> bool {
        self.id == other.id
    }
}

/// The DTO for returning a player
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerWithTeam {
    pub player: Player,
    pub team: Option<Team>,
}
