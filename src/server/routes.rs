use warp::{Reply, Filter, filters::BoxedFilter, http::Response};
use async_graphql::{Schema, Request, http::playground_source, http::GraphQLPlaygroundConfig};
use std::convert::Infallible;

use crate::gql_schema;
use crate::data_base::connection::create_connection_pool;

pub(super) async fn make_routes() -> BoxedFilter<(impl Reply,)>{
    //Build the GraphQL gql_schema
    let schema = gql_schema::build_schema(create_connection_pool().await).finish();

    let graphql_handler = warp::post()
        .and(warp::path("graphql"))
        .and(warp::header::optional::<String>("Authorization"))
        .and(async_graphql_warp::graphql(schema))
        .and_then( |auth: Option<String>, (schema, request): (Schema<_,_,_>, Request)| async move {
            // Do something to get auth data from the header
            let response = schema
                .execute(
                    request
                        //.data(helps::get_user(create_connection_pool(), auth.unwrap_or_default()).await)
                ).await;

            Ok::<_, Infallible>(async_graphql_warp::GraphQLResponse::from(response))
        });

    //GraphQL Playground
    let graphql_playground = warp::path::end().map(||{
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Authorization"]);

    graphql_playground
        .or(graphql_handler)
        .with(cors)
        .boxed()
}