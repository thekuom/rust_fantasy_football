use juniper::Context as JuniperContext;
use juniper::{FieldResult, RootNode};
use uuid::Uuid;

#[macro_use]
pub mod players_api;

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[derive(Debug)]
pub struct Config {
    pub(crate) players_api_host: String,
}

pub struct Context {
    config: Config,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }
}

#[juniper::object(Context = Context)]
impl QueryRoot { 
    // Players API
    fn players(context: &Context) -> FieldResult<Vec<players_api::Player>> {
        players_api::players(context)
    }
    fn player(id: Uuid, context: &Context) -> FieldResult<players_api::Player> {
        players_api::player(id, context)
    }
    fn teams(context: &Context) -> FieldResult<Vec<players_api::Team>> {
        players_api::teams(context)
    }
    fn team(id: Uuid, context: &Context) -> FieldResult<players_api::Team> {
        players_api::team(id, context)
    }
}

#[juniper::object(Context = Context)]
impl MutationRoot { 
    // Players API
    fn create_player(input: players_api::CreatePlayerInput, context: &Context) -> FieldResult<players_api::CreatePlayerResponse> {
        players_api::create_player(input, context)
    }
    fn update_player(input: players_api::UpdatePlayerInput, context: &Context) -> FieldResult<players_api::UpdatePlayerResponse> {
        players_api::update_player(input, context)
    }
    fn delete_player(input: players_api::DeletePlayerInput, context: &Context) -> FieldResult<players_api::DeletePlayerResponse> {
        players_api::delete_player(input, context)
    }
    fn create_team(input: players_api::CreateTeamInput, context: &Context) -> FieldResult<players_api::CreateTeamResponse> {
        players_api::create_team(input, context)
    }
    fn update_team(input: players_api::UpdateTeamInput, context: &Context) -> FieldResult<players_api::UpdateTeamResponse> {
        players_api::update_team(input, context)
    }
    fn delete_team(input: players_api::DeleteTeamInput, context: &Context) -> FieldResult<players_api::DeleteTeamResponse> {
        players_api::delete_team(input, context)
    }
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
