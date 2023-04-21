// Import required modules.
use crate::types::{MutationRoot, QueryRoot, Schema};
use crate::AppContext;
use juniper::{EmptyMutation, EmptySubscription};
use warp::{http::StatusCode, Filter};

// Function to make graphql filter.
pub fn make_graphql_filter(context: AppContext) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create a new schema with the provided query root, mutation root and empty subscription (for our context).
    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::<AppContext>::new());

    // create Juniper GraphQL filter with the specified schema configuration.
    let graphql_filter = juniper_warp::make_graphql_filter(schema, juniper_warp::GraphQLRequest::new,);

    // Create a filter that returns a clone of the provided context.
    let context_filter = warp::any().map(move || context.clone());

    // Combine the two filters and pass it to handle_graphql_request function.
    graphql_filter
        .and(context_filter)
        .and_then(handle_graphql_request)
}

// Async function to handle graphql request.
async fn handle_graphql_request(
    gql_request: juniper_warp::GraphQLRequest,
    context: AppContext,
) -> Result<impl warp::Reply, warp::Rejection> {

    // Get response by executing the request with the provided schema configuration and context.
    let gql_response = gql_request.execute(&Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<AppContext>::new(),
    ), &context);

    // Match json serialization of the response and return appropriate reply.
    match serde_json::to_string(&gql_response) {
        Ok(json) => Ok(warp::reply::with_status(json, StatusCode::OK)),
        Err(_) => Ok(warp::reply::with_status(
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
