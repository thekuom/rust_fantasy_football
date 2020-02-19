use std::collections::HashMap;
use uuid::Uuid;

use crate::teams::models::Team;
use crate::players::models::Player;

pub fn get_players(teams: &HashMap<&str, Team>) -> Vec<Player> {
    let get_team_id = |team: &str| -> Uuid {
        teams.get(team).unwrap().id
    };

    vec![
        Player {
            id: Uuid::new_v4(),
            first_name: "Christian".to_string(),
            last_name: "Kirk".to_string(),
            team_id: Some(get_team_id("cardinals")),
            created_at: None,
            updated_at: None,
        },
        Player {
            id: Uuid::new_v4(),
            first_name: "Kyler".to_string(),
            last_name: "Murray".to_string(),
            team_id: Some(get_team_id("cardinals")),
            created_at: None,
            updated_at: None,
        },
        Player {
            id: Uuid::new_v4(),
            first_name: "Dak".to_string(),
            last_name: "Prescott".to_string(),
            team_id: Some(get_team_id("cowboys")),
            created_at: None,
            updated_at: None,
        },
        Player {
            id: Uuid::new_v4(),
            first_name: "Amari".to_string(),
            last_name: "Cooper".to_string(),
            team_id: Some(get_team_id("cowboys")),
            created_at: None,
            updated_at: None,
        },
        Player {
            id: Uuid::new_v4(),
            first_name: "Eddie".to_string(),
            last_name: "Lacy".to_string(),
            team_id: None,
            created_at: None,
            updated_at: None,
        },
    ]
}
