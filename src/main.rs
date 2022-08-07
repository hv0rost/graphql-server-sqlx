#[macro_use]
//extern crate diesel;
extern crate dotenv;
extern crate log;

mod server;
mod gql_schema;
mod data_base;
mod db_requests;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    server::start(([127,0,0,1], 3030)).await;
}

