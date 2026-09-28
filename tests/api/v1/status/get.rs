use topcoat::router::{Body, StatusCode, request::Request};

#[tokio::test]
async fn return_200() {
    let router = clone_tabnews::pages::router();
    let mut request = Request::new(Body::empty());
    *request.uri_mut() = "/api/v1/status".parse().unwrap();

    let response = router.handle(request).await;

    assert_eq!(response.status(), StatusCode::OK);
}
