/*use std::env;
use diesel::Connection;
use diesel::pg::PgConnection;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}*/


use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async  fn create_connection_pool() -> PgPool {
    PgPool::connect("postgres://postgres:2254@localhost:5432/test").await.unwrap()
}
