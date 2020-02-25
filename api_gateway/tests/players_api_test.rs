mod common;

#[cfg(test)]
mod players_api_tests {
    use crate::common::get_response;
    use fake::{Fake, Faker};
    use mockito::mock;
    use serde_json::json;
    use std::sync::Arc;
    use uuid::Uuid;

    use api_gateway::schema::create_schema;
    use api_gateway::schema::players_api::*;

    #[actix_rt::test]
    async fn test_get_players() {
        let schema = Arc::new(create_schema());

        let payload = json!({
            "query": r#"
                query {
                    players {
                        id
                        firstName
                        lastName
                        team {
                            abbreviation
                        }
                    }
                }
            "#,
        });

        let player_ids: Vec<_> = std::iter::repeat_with(Uuid::new_v4).take(3).collect();
        let team_ids = vec![Some(Uuid::new_v4()), Some(Uuid::new_v4()), None];

        let players: Vec<PlayerWithTeam> = 
            player_ids.iter().zip(team_ids.iter()).map(|(&player_id, &team_id)| {
                // TODO: use faker Dummy derived attribute instead. Requires
                // refactor of using newtype pattern for Uuid
                PlayerWithTeam {
                    player: Player {
                        id: player_id,
                        first_name: Faker.fake(),
                        last_name: Faker.fake(),
                        team_id,
                        team: None,
                    },
                    team: team_id.map(|id| Team {
                        id,
                        display_name: Faker.fake(),
                        abbreviation: Faker.fake(),
                    })
                }
            }).collect();

        let _m = mock("GET", "/players")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!(players).to_string())
            .create();

        let (status, result) = get_response(schema, payload).await;
        assert!(status.is_success());
        assert_eq!(result, json!({
            "data": {
                "players": players.iter().map(|p| json!({
                    "id": p.player.id,
                    "firstName": p.player.first_name,
                    "lastName": p.player.last_name,
                    "team": p.team.as_ref().map(|team| {
                        json!({
                            "abbreviation": team.abbreviation
                        })
                    })
                })).collect::<Vec<_>>()
            }
        }));
    }

    // TODO: fill out more tests
}
