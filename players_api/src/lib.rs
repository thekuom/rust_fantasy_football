/// lib file. Sets up the web server

/*
 * Because we put main.rs in src/bin, src/lib.rs becomes
 * the entrypoint for the application. This is common
 * practice so we can import functions from here in
 * our integration tests.
 */

// Import the macros in the diesel crate i.e. Associations, Queryable, etc
#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, FromRequest, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub mod common;
pub mod players;
pub mod schema;
pub mod seeds;
pub mod teams;

use common::DeserializeErrorHandler;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Contains data that is passed to every request and is
/// shared with all requests
pub struct AppData {
    /// Pool of postgres database connections
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn register(db_pool: PgPool) -> impl Fn(&mut web::ServiceConfig) {
    move |config: &mut web::ServiceConfig| {
        use crate::players::models::{CreatePlayerForm, UpdatePlayerForm};

        config
            .data(AppData { db_pool: db_pool.clone() })
            .service(
                web::resource("/players")
                .app_data(
                    web::Json::<CreatePlayerForm>::configure(CreatePlayerForm::handle_deserialize)
                )
                .route(web::get().to(players::get_players))
                .route(web::post().to(players::create_player))
            )
            .service(
                web::resource("/players/{id}")
                .app_data(
                    web::Json::<UpdatePlayerForm>::configure(UpdatePlayerForm::handle_deserialize)
                )
                .route(web::get().to(players::get_player))
                .route(web::put().to(players::update_player))
                .route(web::delete().to(players::delete_player))
            )
            .service(
                web::resource("/teams")
                .route(web::get().to(teams::get_teams))
                .route(web::post().to(teams::create_team))
            )
            .service(
                web::resource("/teams/{id}")
                .route(web::get().to(teams::get_team))
                .route(web::put().to(teams::update_team))
                .route(web::delete().to(teams::delete_team))
            );
    }
}

/// Sets up the web server
///
/// # Panics
///
/// Panics if `.env` cannot be parsed
///
/// Panics if environment variable `DATABASE_URL` is not set
///
/// Panics if it fails to create database pool
pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // Reads the .env file nad makes sure it is parsable
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(register(pool.clone()))
    })
        .bind("0.0.0.0:4000")?
        .workers(2)
        .run()
        .await
}
