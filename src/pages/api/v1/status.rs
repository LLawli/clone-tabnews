use crate::infra::database;
use serde_json::{Value, json};
use topcoat::{
    Result,
    router::{StatusCode, content::Json, response::IntoResponse, route},
};

#[route(GET)]
async fn status() -> Result<impl IntoResponse> {
    let result = database::query("SELECT 1+1 AS soma;", &[]).await?;
    let soma: i32 = result[0].get("soma");
    Ok((
        StatusCode::OK,
        [("content-type", "application/json; charset=utf-8")],
        Json(json!({
            "status": "ok",
            "database": {"soma": soma}
        })),
    ))
}
