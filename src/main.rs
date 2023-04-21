// Import the juniper macro for derive macros and to avoid importing other macros one by one
#[macro_use]
extern crate juniper;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

// Import necessary items from the juniper crate
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::env;
use std::sync::Arc;

// Import local modules
mod config; // Contains application configuration
mod models; // Contains data models
mod neo4j; // Contains Neo4j database connectivity functions
mod resolvers; // Contains GraphQL query and mutation resolver functions
mod schema; // Defines the app schema
mod types; // Define custom scalar types used in the app
mod utils; // Contains utility functions used throughout the app

// Import items from local modules
use crate::config::AppContext;
use crate::resolvers::{MutationRoot, QueryRoot};
use crate::schema::Schema;

async fn graphql_handler(
    st: web::Data<Arc<AppContext>>,
    data: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let schema = Schema::new(QueryRoot {}, MutationRoot {});
    
    // Execute the input GraphQL query using the app schema and context
    let res = data.execute(&schema, &st);
    
    // Serialize the query result into JSON format for sending over HTTP
    let json_result = serde_json::to_string(&res).unwrap();
    
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_result)
}

async fn graphiql_handler() -> HttpResponse {
    // Generate HTML for the GraphiQL interface to test the GraphQL API
    let html = graphiql_source("/graphql", None);
    
    // Create an HTTP response with the HTML
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the server port from an environment variable or use the default value 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Read Neo4j database URI, username, and password from environment variables
    let neo4j_uri = env::var("NEO4J_URI").expect("NEO4J_URI environment variable must be set");
    let neo4j_user = env::var("NEO4J_USER").expect("NEO4J_USER environment variable must be set");
    let neo4j_password =
        env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD environment variable must be set");

    // Create a Neo4jConfig struct with the database configuration values
    let neo4j_config = neo4j::Neo4jConfig {
        uri: neo4j_uri,
        user: neo4j_user,
        password: neo4j_password,
    };

    // Create a Neo4j driver and client instances using the config
    let neo4j_driver = neo4j::create_driver(neo4j_config);
    let neo4j_client = neo4j::create_client(neo4j_driver);

    // Create app context object containing the neo4j_client
    let app_context = Arc::new(AppContext { neo4j_client });

    HttpServer::new(move || {
        App::new()
            .data(app_context.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(graphiql_handler))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
