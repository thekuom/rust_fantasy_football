use juniper::{FieldResult, GraphQLObject};
use serde::Deserialize;
use uuid::Uuid;

use super::{Context, QueryRoot};

#[derive(GraphQLObject, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub display_name: String,
    pub abbreviation: String,
}

#[derive(Deserialize, GraphQLObject)]
pub struct Player {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,

    #[graphql(skip)]
    pub team_id: Option<Uuid>,

    #[serde(skip_deserializing)]
    pub team: Option<Team>,
}

#[derive(Deserialize)]
pub struct PlayerWithTeam {
    pub player: Player,
    pub team: Option<Team>,
}

impl QueryRoot {
    pub fn players_impl(context: &Context) -> FieldResult<Vec<Player>> {
        let api = &context.config.players_api_host;
        let client = reqwest::blocking::Client::new();

        // TODO: handle errors
        let players: Vec<_> = client
            .get(format!("{}/players", api).as_str())
            .send()
            .unwrap()
            .json::<Vec<PlayerWithTeam>>()
            .unwrap()
            .into_iter()
            .map(|player_with_team| {
                Player {
                    team: player_with_team.team,
                    ..player_with_team.player
                }
            })
            .collect();

        Ok(players)
    }

    pub fn player_impl(id: String, context: &Context) -> FieldResult<Player> {
        let api = &context.config.players_api_host;
        let client = reqwest::blocking::Client::new();

        // TODO: handle errors
        let player = client
            .get(format!("{}/players/{}", api, id).as_str())
            .send()
            .unwrap()
            .json::<PlayerWithTeam>()
            .map(|player_with_team| {
                Player {
                    team: player_with_team.team,
                    ..player_with_team.player
                }
            })
            .unwrap();

        Ok(player)
    }
}
