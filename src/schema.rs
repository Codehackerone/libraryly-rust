// Importing required modules.
use crate::models::{Book, NewBookInput};
use crate::neo4j::neo4j_client;
use juniper::{FieldResult, RootNode};

// Define QueryRoot struct.
pub struct QueryRoot;

// Implement GraphQL object for QueryRoot.
#[juniper::graphql_object(Context = AppContext)]
impl QueryRoot {

    // Async function to return a FieldResult containing a Book with provided id.
    async fn book(id: String) -> FieldResult<Book> {

        // Get Neo4j client.
        let client = neo4j_client().await?;

        // Construct cypher query to get book information based on provided ID.
        let query = format!("MATCH (b:Book {{id: '{}'}}) RETURN b.id AS id, b.title AS title, b.author AS author, b.description AS description", id);

        // Execute the cypher query and get the result.
        let result = client.execute_statement(query).await?;

        // Extract record from the result and map fields to corresponding variables.
        let record = &result.records().get(0).unwrap();
        let id = record.get("id").unwrap().to_string();
        let title = record.get("title").unwrap().to_string();
        let author = record.get("author").unwrap().to_string();
        let description = record.get("description").map(|v| v.to_string());

        // Return Book object as FieldResult.
        Ok(Book {
            id,
            title,
            author,
            description,
        })
    }
}

// Define MutationRoot struct.
pub struct MutationRoot;

// Implement GraphQL object for MutationRoot.
#[juniper::graphql_object(Context = AppContext)]
impl MutationRoot {

    // Async function to create a new book with provided input and return it as FieldResult.
    async fn create_book(input: NewBookInput) -> FieldResult<Book> {

        // Get Neo4j client.
        let client = neo4j_client().await?;

        // Create new ID for the book.
        let id = uuid::Uuid::new_v4().to_string();

        // Extract fields from input object.
        let title = input.title;
        let author = input.author;
        let description = input.description;

        // Construct cypher query to create a new book node in the database and return its information.
        let query = format!("CREATE (b:Book {{id: '{}', title: '{}', author: '{}', description: '{}' }}) RETURN b.id AS id, b.title AS title, b.author AS author, b.description AS description", id, title, author, description.unwrap_or_default());

        // Execute the cypher query and get the result.
        let result = client.execute_statement(query).await?;

        // Extract record from the result and map fields to corresponding variables.
        let record = &result.records().get(0).unwrap();
        let id = record.get("id").unwrap().to_string();
        let title = record.get("title").unwrap().to_string();
        let author = record.get("author").unwrap().to_string();
        let description = record.get("description").map(|v| v.to_string());

        // Return Book object as FieldResult.
        Ok(Book {
            id,
            title,
            author,
            description,
        })
    }
}

// Define schema using RootNode with QueryRoot and MutationRoot structs.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
