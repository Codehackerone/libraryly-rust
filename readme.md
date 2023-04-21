# libraryly-rust
This API allows you to manage a library of books. You can add, edit, and delete books, as well as search for books by a variety of criteria. An attempt to make 
[libraryly](https://github.com/Codehackerone/Libraryly) in rust using graphql and neo4j database

# Base URL
The base URL for this API is http://localhost:8000/.

# Authentication
This API does not require authentication.

# Endpoints

`GET /graphql` - 
This endpoint allows you to query the library database using GraphQL.

Query Parameters
query: The GraphQL query to execute.
Response
The response will be a JSON object containing the data requested by the query, or an error message if the query failed. The exact response format will depend on the query.