// Import the Neo4jClient struct from the neo4rs crate
use neo4rs::Neo4jClient;
// Import the std library's env module for working with environment variables
use std::env;

// Define a Config struct with public fields for the Neo4j client and port number
pub struct Config {
    pub neo4j_client: Neo4jClient,
    pub port: u16,
}

impl Config {
    // Define an associated function called `from_env` which returns a Config object
    pub fn from_env() -> Config {
        // Try to get the value of the NEO4J_URL environment variable, or use a default value if it is not set
        let neo4j_url = env::var("NEO4J_URL").unwrap_or("bolt://localhost:7687".to_string());
        // Try to get the value of the NEO4J_USERNAME environment variable, or use a default value if it is not set
        let neo4j_username = env::var("NEO4J_USERNAME").unwrap_or("neo4j".to_string());
        // Try to get the value of the NEO4J_PASSWORD environment variable, or use a default value if it is not set
        let neo4j_password = env::var("NEO4J_PASSWORD").unwrap_or("password".to_string());
        // Create a new Neo4jClient object by connecting to the database using the URL, username, and password
        let neo4j_client = Neo4jClient::connect(&neo4j_url, &neo4j_username, &neo4j_password).unwrap();
        
        // Return a new Config object with the Neo4j client and port number
        // The port number is obtained by trying to get the PORT environment variable, or using a default value if it is not set
        Config {
            neo4j_client,
            port: env::var("PORT").unwrap_or("8000".to_string()).parse().unwrap(),
        }
    }
}
