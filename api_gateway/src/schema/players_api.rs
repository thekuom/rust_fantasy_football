use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Context;

/// Additional functions on Result which will help with handling
/// responses
trait ReqwestResponse {
    fn handle_response_errors(self) -> reqwest::Result<Result<reqwest::blocking::Response, String>>;
}

impl ReqwestResponse for reqwest::Result<reqwest::blocking::Response> {
    /// Handles response status errors i.e. 400. The players_api will return errors with the
    /// JSON body { "message": String, "data": Object }. This function will extract the message
    /// part and return it as an error
    fn handle_response_errors(self) -> reqwest::Result<Result<reqwest::blocking::Response, String>> {
        self.map(|res| {
            if !res.status().is_success() {
                let err_message = res.json::<JsonError>()
                    .map_or("Something went wrong with the request".to_string(), |json_err| {
                        json_err.message
                    });

                return Err(err_message);
            }

            Ok(res)
        })
    }
}

macro_rules! PlayerBase {
    ($(#[$attr:meta])* $name:ident { $( $(#[$field_attr:meta])* $field:ident: $type:ty ),* $(,)?}) => {
        $(#[$attr])*
        pub struct $name {
            pub first_name: String,
            pub last_name: String,
            pub team_id: Option<Uuid>,

            $(
                $(#[$field_attr])*
                pub $field: $type 
            ),*
        }
    }
}

#[derive(Deserialize)]
pub struct JsonError {
    message: String,
}

#[derive(Debug, GraphQLObject, Clone, Deserialize, Serialize)]
pub struct Team {
    pub id: Uuid,
    pub display_name: String,
    pub abbreviation: String,
}

PlayerBase!(
    #[derive(Debug, Deserialize, Serialize)]
    Player {
        id: Uuid,

        #[serde(skip_deserializing)]
        team: Option<Team>,
    }
);

#[juniper::object(Context = Context)]
impl Player {
    // TODO: when Juniper implements merging GraphQL objects,
    // we will not have to do these verbose getters
    fn id(&self) -> &Uuid { &self.id } 
    fn first_name(&self) -> &str { &self.first_name }
    fn last_name(&self) -> &str { &self.last_name }

    /// Gets the team of the player. For read methods,
    /// the team is returned with the player so we can skip the request
    /// to the players API. If the team is not returned but the player
    /// has a team_id, fetch the team from the API
    fn team(&self, context: &Context) -> FieldResult<Option<Team>> {
        match (self.team_id, self.team.as_ref()) {
            (Some(team_id), Some(team)) => Ok(Some(team.clone())),
            (None, _) => Ok(None),
            (Some(team_id), None) => team(team_id, context).map(Some),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlayerWithTeam {
    pub player: Player,
    pub team: Option<Team>,
}

pub fn players(context: &Context) -> FieldResult<Vec<Player>> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let players: Vec<_> = client
        .get(format!("{}/players", api).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error fetching the players".to_string())
        })?
        .json::<Vec<PlayerWithTeam>>()?
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

pub fn player(id: Uuid, context: &Context) -> FieldResult<Player> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let player = client
        .get(format!("{}/players/{}", api, id).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error fetching the player".to_string())
        })?
        .json::<PlayerWithTeam>()
        .map(|player_with_team| {
            Player {
                team: player_with_team.team,
                ..player_with_team.player
            }
        })?;

    Ok(player)
}

PlayerBase!(
    #[derive(GraphQLInputObject, Serialize)]
    CreatePlayerInput { }
);

pub struct CreatePlayerResponse {
    pub player: Player,
}

#[juniper::object(Context = Context)]
impl CreatePlayerResponse {
    fn player(&self) -> &Player { &self.player }
}

pub fn create_player(input: CreatePlayerInput, context: &Context) -> FieldResult<CreatePlayerResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let player = client
        .post(format!("{}/players", api).as_str())
        .json(&input)
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error creating the player".to_string())
        })?
        .json::<Player>()?;

    Ok(CreatePlayerResponse { player })
}

PlayerBase!(
    #[derive(GraphQLInputObject, Serialize)]
    UpdatePlayerInput { 
        #[serde(skip_serializing)]
        id: Uuid,
    }
);

pub struct UpdatePlayerResponse {
    pub player: Player,
}

#[juniper::object(Context = Context)]
impl UpdatePlayerResponse {
    fn player(&self) -> &Player { &self.player }
}

pub fn update_player(input: UpdatePlayerInput, context: &Context) -> FieldResult<UpdatePlayerResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let player = client
        .put(format!("{}/players/{}", api, input.id).as_str())
        .json(&input)
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error updating the player".to_string())
        })?
        .json::<Player>()?;

    Ok(UpdatePlayerResponse { player })
}

#[derive(GraphQLInputObject)]
pub struct DeletePlayerInput {
    pub id: Uuid,
}

#[derive(GraphQLObject)]
pub struct DeletePlayerResponse {
    pub success: bool,
}

pub fn delete_player(input: DeletePlayerInput, context: &Context) -> FieldResult<DeletePlayerResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    client
        .delete(format!("{}/players/{}", api, input.id).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error deleting the player".to_string())
        })?;

    Ok(DeletePlayerResponse { success: true })
}

pub fn teams(context: &Context) -> FieldResult<Vec<Team>> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let teams: Vec<_> = client
        .get(format!("{}/teams", api).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error fetching the teams".to_string())
        })?
        .json::<Vec<Team>>()?
        .into_iter()
        .collect();

    Ok(teams)
}

pub fn team(id: Uuid, context: &Context) -> FieldResult<Team> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let team = client
        .get(format!("{}/teams/{}", api, id).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error fetching the team".to_string())
        })?
        .json::<Team>()?;

    Ok(team)
}

#[derive(GraphQLInputObject, Serialize)]
pub struct CreateTeamInput {
    pub display_name: String,
    pub abbreviation: String,
}

pub struct CreateTeamResponse {
    pub team: Team,
}

#[juniper::object(Context = Context)]
impl CreateTeamResponse {
    fn team(&self) -> &Team { &self.team }
}

pub fn create_team(input: CreateTeamInput, context: &Context) -> FieldResult<CreateTeamResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let team = client
        .post(format!("{}/teams", api).as_str())
        .json(&input)
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error creating the team".to_string())
        })?
        .json::<Team>()?;

    Ok(CreateTeamResponse { team })
}

#[derive(GraphQLInputObject, Serialize)]
pub struct UpdateTeamInput {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub display_name: String,
    pub abbreviation: String,
}

pub struct UpdateTeamResponse {
    pub team: Team,
}

#[juniper::object(Context = Context)]
impl UpdateTeamResponse {
    fn team(&self) -> &Team { &self.team }
}

pub fn update_team(input: UpdateTeamInput, context: &Context) -> FieldResult<UpdateTeamResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    let team = client
        .put(format!("{}/teams/{}", api, input.id).as_str())
        .json(&input)
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error updating the team".to_string())
        })?
        .json::<Team>()?;

    Ok(UpdateTeamResponse { team })
}

#[derive(GraphQLInputObject)]
pub struct DeleteTeamInput {
    pub id: Uuid,
}

#[derive(GraphQLObject)]
pub struct DeleteTeamResponse {
    pub success: bool,
}

pub fn delete_team(input: DeleteTeamInput, context: &Context) -> FieldResult<DeleteTeamResponse> {
    let api = &context.config.players_api_host;
    let client = reqwest::blocking::Client::new();

    client
        .delete(format!("{}/teams/{}", api, input.id).as_str())
        .send()
        .handle_response_errors()
        .unwrap_or_else(|_err| {
            Err("There was an error deleting the team".to_string())
        })?;

    Ok(DeleteTeamResponse { success: true })
}
