use crate::common;
use chrono::DateTime;
use topcoat::router::{Body, StatusCode, request::Request};

#[tokio::test]
async fn return_200_and_correct_body() {
    common::setup();
    let router = clone_tabnews::pages::router();
    let mut request = Request::new(Body::empty());
    *request.uri_mut() = "/api/v1/status".parse().unwrap();

    let response = router.handle(request).await;

    let parsed_response = common::process_response(response).await;
    assert_eq!(parsed_response.status_code, StatusCode::OK);
    let body = parsed_response.body.as_ref().unwrap();
    let database_version = body["dependencies"]["database"]["version"]
        .as_str()
        .expect("database version não existe ou não é uma string");
    let database_max_connections = body["dependencies"]["database"]["max_connections"]
        .as_u64()
        .expect("database max_connections não existe ou não é um inteiro não negativo");
    let database_connections = body["dependencies"]["database"]["opened_connections"]
        .as_u64()
        .expect("database opened_connections não existe ou não é um inteiro não negativo");
    let updated_at_str = body["updated_at"]
        .as_str()
        .expect("updated_at não existe ou não é uma string");
    let parse_result = DateTime::parse_from_rfc3339(updated_at_str);
    assert!(
        parse_result.is_ok(),
        "O campo updated_at '{}' não está no formato ISO 8601 válido",
        updated_at_str
    );
    assert_eq!(database_version, "16.0");
    assert!(
        database_max_connections >= database_connections,
        "Existem mais conexões abertas do que o permitido no banco de dados."
    )
}
