use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use diesel::Connection;
use dotenv::dotenv;
use std::env;
pub struct Db;

// Define a type alias for the connection pool
type PgPool = Pool<ConnectionManager<PgConnection>>;

impl Db {
    pub fn create_db_pool() -> PgPool {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Create a connection manager
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        // Create the connection pool
        Pool::builder()
            .build(manager)
            .expect("Failed to create database pool")
    }

    pub fn get_db_connection(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
        pool.get()
            .expect("Failed to get a database connection from the pool")
    }

    pub fn connect_to_db() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
