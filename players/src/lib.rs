#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub mod players;
pub mod schema;

pub struct AppData { 
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || 
        App::new()
            .data(AppData { db_pool: pool.clone() })
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/players").route(web::get().to(players::get_players))
            )
        )
    .bind("0.0.0.0:4000")?
    .workers(2)
    .run()
    .await
}
