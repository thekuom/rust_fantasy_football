use serde::{Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::schema::{players, teams};

#[derive(Associations, Identifiable, PartialEq, Serialize, Queryable)]
#[belongs_to(Team)]
#[table_name = "players"]
pub struct Player {
    pub id: Uuid, 
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub team_id: Option<Uuid>,
}

#[derive(Identifiable, Serialize, PartialEq, Queryable)]
#[table_name = "teams"]
pub struct Team {
    pub id: Uuid,
    pub display_name: String,
    pub abbreviation: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}
