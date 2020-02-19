use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use common_derive::DeserializeErrorHandler;
use crate::schema::teams;

/// Team model. Represents a team a player can be on
#[derive(Identifiable, Insertable, Debug, Deserialize, Serialize, Queryable)]
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

#[derive(Insertable, Debug, Deserialize, DeserializeErrorHandler, Serialize)]
#[table_name = "teams"]
pub struct CreateTeamForm {
    pub display_name: String,
    pub abbreviation: String,
}

#[derive(AsChangeset, Debug, Deserialize, DeserializeErrorHandler, Serialize)]
#[table_name = "teams"]
pub struct UpdateTeamForm {
    pub display_name: String,
    pub abbreviation: String,
}
