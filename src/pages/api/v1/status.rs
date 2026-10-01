use crate::infra::database;
use serde_json::{Value, json};
use topcoat::{
    Result,
    router::{
        StatusCode,
        content::Json,
        error::{RouterErrorExt, internal_server_error},
        response::IntoResponse,
        route,
    },
};

#[route(GET)]
async fn status() -> Result<impl IntoResponse> {
    Ok(match collect_status().await {
        Ok(body) => (StatusCode::OK, Json(body)),
        Err(e) => {
            eprintln!("database error: {e}");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "database": "unavailable" })),
            )
        }
    })
}

async fn collect_status() -> Result<serde_json::Value> {
    const STATUS_SQL: &str = "SELECT current_setting('server_version') AS server_version, \
        current_setting('max_connections')::int AS max_connections, \
        count(*)::int AS opened_connections FROM pg_stat_activity WHERE datname = current_database();";
    let updated_at = chrono::Utc::now().to_rfc3339();
    let rows = database::query(STATUS_SQL, &[])
        .await
        .map_err(internal_server_error)?;
    let row = rows.first().ok_or_else(|| {
        internal_server_error(std::io::Error::other("status query returned no rows"))
    })?;

    let version: String = row.try_get("server_version")?;
    let max_connections: i32 = row.try_get("max_connections")?;
    let opened_connections: i32 = row.try_get("opened_connections")?;

    Ok(json!(
        {
            "updated_at": updated_at,
            "dependencies": {
                "database": {
                    "version": version,
                    "max_connections": max_connections,
                    "opened_connections": opened_connections
                }
            }
        }
    ))
}
