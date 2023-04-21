// Import necessary items from external crates
use neo4rs::*;
use std::env;

// Define an asynchronous function to connect to Neo4j database
pub async fn neo4j_client() -> neo4rs::Neo4jResult<neo4rs::Neo4jClient> {
    // Retrieve Neo4j URI, username and password from environment variables
    let uri = env::var("NEO4J_URI").expect("NEO4J_URI not found in environment variables");
    let username = env::var("NEO4J_USERNAME").expect("NEO4J_USERNAME not found in environment variables");
    let password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD not found in environment variables");

    // Create a new `AuthToken` with the retrieved credentials
    let auth_token = neo4rs::AuthToken::basic(&username, &password);

    // Configure client options with the URI and `AuthToken`
    let options = neo4rs::Neo4jOptions::new(uri.as_str()).auth_token(auth_token);

    // Connect to the Neo4j instance using the configured options
    neo4rs::Neo4jClient::connect(options).await
        .map_err(|e| neo4rs::Neo4jError::from(format!("Failed to connect to Neo4j: {:?}", e)))
}
