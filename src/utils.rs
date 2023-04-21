// Import necessary items from external crates
use neo4rs::model::Node;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Define struct to hold book data
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub genre: String,
}

// Implement the `From` trait to convert a Neo4j `Node` into a `Book` instance
impl From<Node> for Book {
    fn from(node: Node) -> Self {
        // Extract book properties from the `Node` object and create a new `Book`
        Book {
            id: node.id().to_string(),
            title: node.properties().get("title").unwrap().to_string(),
            author: node.properties().get("author").unwrap().to_string(),
            genre: node.properties().get("genre").unwrap().to_string(),
        }
    }
}

// Implement methods for the `Book` struct
impl Book {
    // Constructor method to create a new `Book` with a unique ID
    pub fn new(title: String, author: String, genre: String) -> Book {
        Book {
            id: Uuid::new_v4().to_string(), // Generate a new UUID as string for the `id` field
            title,
            author,
            genre,
        }
    }
}
