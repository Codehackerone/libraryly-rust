// Import the necessary structs from the juniper crate
use juniper::{GraphQLInputObject, GraphQLObject};

// Define a Book struct with fields for the book's id, title, author, and description
// Derive the GraphQLObject trait so instances of Book can be returned in GraphQL queries
#[derive(GraphQLObject)]
#[graphql(description = "A book in the library")]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
}

// Define a NewBookInput struct with fields for the new book's title, author, and description
// Derive the GraphQLInputObject trait so instances of NewBookInput can be used as input arguments in GraphQL mutations
#[derive(GraphQLInputObject)]
#[graphql(description = "New book input")]
pub struct NewBookInput {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
}
