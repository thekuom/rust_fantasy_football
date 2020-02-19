/// The models needed for the players APIs

// Deserialize and Serialize help translate to and from JSON
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use common_derive::DeserializeErrorHandler;
use crate::schema::players;
use crate::teams::models::Team;

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

#[derive(Insertable, Debug, Deserialize, DeserializeErrorHandler, Serialize)]
#[table_name = "players"]
pub struct CreatePlayerForm {
    pub first_name: String,
    pub last_name: String,
    pub team_id: Option<Uuid>,
}

#[changeset_options(treat_none_as_null="true")]
#[derive(AsChangeset, Debug, Deserialize, DeserializeErrorHandler, Serialize)]
#[table_name = "players"]
pub struct UpdatePlayerForm {
    pub first_name: String,
    pub last_name: String,
    pub team_id: Option<Uuid>,
}

/// The DTO for returning a player
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerWithTeam {
    pub player: Player,
    pub team: Option<Team>,
}
