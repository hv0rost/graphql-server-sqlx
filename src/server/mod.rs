use std::net::SocketAddr;

mod routes;

//Start the API server
pub async fn start(addr : impl Into<SocketAddr>){
    warp::serve(routes::make_routes().await).run(addr).await;
}