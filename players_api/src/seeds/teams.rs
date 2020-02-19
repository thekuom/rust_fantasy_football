use std::collections::HashMap;
use uuid::Uuid;

use crate::teams::models::Team;

pub fn get_teams() -> HashMap<&'static str, Team> {
    vec![
        ("cardinals", Team {
            id: Uuid::new_v4(),
            display_name: "Arizona Cardinals".to_string(),
            abbreviation: "ARI".to_string(),
            created_at: None,
            updated_at: None,
        }),
        ("cowboys", Team {
            id: Uuid::new_v4(),
            display_name: "Dallas Cowboys".to_string(),
            abbreviation: "DAL".to_string(),
            created_at: None,
            updated_at: None,
        }),
        ("packers", Team {
            id: Uuid::new_v4(),
            display_name: "Green Bay Packers".to_string(),
            abbreviation: "GB".to_string(),
            created_at: None,
            updated_at: None,
        }),
    ].into_iter().collect()
}
