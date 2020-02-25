#[cfg(feature = "test")]
use mockito;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

pub mod schema;

use crate::schema::{create_schema, Schema};

async fn playground() -> HttpResponse {
    let html = playground_source("http://localhost:4000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[cfg(not(feature = "test"))]
fn get_config() -> schema::Config {
    let players_api_host = std::env::var("PLAYERS_API_URL").expect("PLAYERS_API_URL must be set");

    schema::Config { players_api_host }
}

#[cfg(feature = "test")]
fn get_config() -> schema::Config {
    let players_api_host = mockito::server_url();

    schema::Config { players_api_host }
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

    let config = get_config();
    let ctx = schema::Context::new(config);

    let result = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

pub fn register(schema: Arc<schema::Schema>) -> impl Fn(&mut web::ServiceConfig) {
    move |config: &mut web::ServiceConfig| {
        config
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/playground").route(web::get().to(playground)));
    }
}

pub async fn run() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    dotenv().ok();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .max_age(3600)
                    .finish()
            )
            .wrap(middleware::Logger::default())
            .configure(register(schema.clone()))
    })
    .bind("0.0.0.0:4000")?
    .workers(3)
    .run()
    .await
}
