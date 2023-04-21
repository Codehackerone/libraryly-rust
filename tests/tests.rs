#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_graphql_handler() {
        let app_context = Arc::new(AppContext {
            neo4j_client: neo4j::create_test_client(),
        });
        let data = web::Data::new(app_context);
        let query = r#"
            query {
                books {
                    id
                    title
                    author {
                        id
                        name
                    }
                }
            }
        "#;
        let request = GraphQLRequest::new(query.to_owned(), None, None);
        let res = graphql_handler(data, web::Json(request)).await;
        assert_eq!(res.status(), 200);
        let body = test::read_body(res).await;
        assert!(body.contains("Harry Potter and the Philosopher's Stone"));
    }

    #[actix_rt::test]
    async fn test_graphiql_handler() {
        let mut app = test::init_service(App::new().route("/", web::get().to(graphiql_handler)))
            .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res.status(), 200);
        let body = test::read_body(res).await;
        assert!(body.contains("<title>GraphiQL</title>"));
    }
}
