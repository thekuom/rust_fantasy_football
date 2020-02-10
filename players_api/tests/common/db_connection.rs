use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager, PoolError };
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes the database pool
fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

/// Gets the pool for the test database
///
/// # Panics
///
/// Panics when `.env` cannot be parsed
///
/// Panics when `DATABASE_URL_TEST` is not set
///
/// Panics when it fails to create the database pool
pub fn get_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
    init_pool(database_url.as_str()).expect("Failed to create db pool")
}
