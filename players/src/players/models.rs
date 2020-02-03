use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::schema::{players, teams};

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
