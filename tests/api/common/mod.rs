use serde_json::Value;
use std::sync::Once;
use topcoat::router::{HeaderMap, StatusCode, response::Response, to_bytes};

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.development").ok();
    });
}

pub struct ParsedResponse {
    pub status_code: StatusCode,
    pub body: Option<Value>,
    pub headers: HeaderMap,
}

pub async fn process_response(response: Response) -> ParsedResponse {
    let (parts, body) = response.into_parts();
    ParsedResponse {
        status_code: parts.status,
        body: serde_json::from_slice(&to_bytes(body, usize::MAX).await.unwrap()).ok(),
        headers: parts.headers,
    }
}
