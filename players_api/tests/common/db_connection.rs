use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager, PoolError};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes the database pool
fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

struct DbPool {
    pool: Option<PgPool>,
}

impl DbPool {
    /// Gets the pool for the test database
    ///
    /// # Panics
    ///
    /// Panics when `.env` cannot be parsed
    ///
    /// Panics when `DATABASE_URL_TEST` is not set
    ///
    /// Panics when it fails to create the database pool
    fn get_pool(&mut self) -> &PgPool {
        if self.pool.is_some() {
            return self.pool.as_ref().unwrap();
        }

        self.pool = {
            dotenv().ok();
            let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");

            Some(init_pool(database_url.as_str()).expect("Failed to create db pool"))
        };

        self.get_pool()
    }
}

static mut DB_POOL: DbPool = DbPool {
    pool: None,
};

pub fn get_pool<'a>() -> &'a PgPool {
   unsafe { DB_POOL.get_pool() }
}
