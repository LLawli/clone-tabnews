use crate::infra::database;
use serde_json::{Value, json};
use topcoat::{
    Result,
    router::{StatusCode, content::Json, response::IntoResponse, route},
};

#[route(GET)]
async fn status() -> Result<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        [("content-type", "application/json; charset=utf-8")],
        Json(json!({
            "status": "ok"
        })),
    ))
}
