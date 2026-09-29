use serde_json::Value;
use topcoat::router::{Body, StatusCode, content::Json, request::Request, to_bytes};

use crate::common;

#[tokio::test]
async fn return_200_and_correct_body() {
    common::setup();
    let router = clone_tabnews::pages::router();
    let mut request = Request::new(Body::empty());
    *request.uri_mut() = "/api/v1/status".parse().unwrap();

    let response = router.handle(request).await;

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["database"]["soma"], 2);
}
