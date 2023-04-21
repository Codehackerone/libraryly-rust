// Importing required modules.
use crate::utils::Book;
use juniper::{graphql_object, FieldResult};

// Define QueryRoot struct.
#[derive(Debug, Clone)]
pub struct QueryRoot;

// Implement GraphQL object for QueryRoot.
#[graphql_object(Context = AppContext)]
impl QueryRoot {

    // Function to get API version.
    fn apiVersion() -> &str {
        "1.0"
    }

    // Function to get all books from the database.
    fn books(context: &AppContext) -> FieldResult<Vec<Book>> {

        // Construct cypher query to get all book nodes in the database.
        let query = "MATCH (b:Book) RETURN b";

        // Execute the cypher query and get the result.
        let result = context.neo4j_client.cypher(query, None)?;

        // Iterate over each row in the result and map book node to Book struct.
        let books = result.rows().iter().map(|row| {
            let book_node: Node = row.get("b").unwrap();
            Book::from(book_node)
        }).collect();

        // Return vector of Books as FieldResult.
        Ok(books)
    }
}

// Define MutationRoot struct.
#[derive(Debug, Clone)]
pub struct MutationRoot;

// Implement GraphQL object for MutationRoot.
#[graphql_object(Context = AppContext)]
impl MutationRoot {

    // Function to add a new book to the database.
    fn addBook(context: &AppContext, title: String, author: String, genre: String) -> FieldResult<Book> {

        // Create new Book object with provided title, author, and genre.
        let book = Book::new(title, author, genre);

        // Construct cypher query to create a new book node in the database and return its information.
        let query = "CREATE (b:Book { id: $id, title: $title, author: $author, genre: $genre }) RETURN b";

        // Create a HashMap of parameters to be used in the cypher query.
        let mut params = std::collections::HashMap::new();
        params.insert("id".to_string(), book.id.clone().into());
        params.insert("title".to_string(), book.title.clone().into());
        params.insert("author".to_string(), book.author.clone().into());
        params.insert("genre".to_string(), book.genre.clone().into());

        // Execute the cypher query and get the result.
        let result = context.neo4j_client.cypher(query, Some(params))?;

        // Extract book node from the result and map it to Book struct.
        let book_node: Node = result.rows()[0].get("b").unwrap();
        let book = Book::from(book_node);

        // Return Book object as FieldResult.
        Ok(book)
    }
}

// Define schema using RootNode with QueryRoot and MutationRoot structs.
pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot>;
