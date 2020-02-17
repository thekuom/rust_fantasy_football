use juniper::{FieldResult, RootNode};

pub mod players_api;

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub struct Config {
    pub(crate) players_api_host: String,
}

pub struct Context {
    config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }
}

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn players(context: &Context) -> FieldResult<Vec<players_api::Player>> {
        QueryRoot::players_impl(context)
    }

    fn player(id: String, context: &Context) -> FieldResult<players_api::Player> {
        QueryRoot::player_impl(id, context)
    }
}

#[juniper::object(Context = Context)]
impl MutationRoot { 
    fn echo(context: &Context) -> &'static str {
        "echo"
    }
}

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
