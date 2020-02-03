use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager, PoolError };
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
    init_pool(database_url.as_str()).expect("Failed to create db pool")
}
