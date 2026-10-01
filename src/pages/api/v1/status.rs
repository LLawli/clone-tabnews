use crate::infra::database::query;
use serde_json::json;
use topcoat::{
    Result,
    router::{StatusCode, content::Json, response::IntoResponse, route},
};

#[route(GET)]
async fn status() -> Result<impl IntoResponse> {
    let updated_at = chrono::offset::Utc::now().to_rfc3339();
    let database_status_query = match query("SELECT current_setting('server_version') AS server_version, \
                                                        current_setting('max_connections')::int AS max_connections, \
                                                        count(*)::int AS opened_connections FROM pg_stat_activity WHERE datname = current_database();", &[]).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("erro na query de status: {e:?}");
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))))
        }
    };
    let Some(row) = database_status_query.first() else {
        return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))));
    };
    let version: String = row.get("server_version");
    let max_connections: i32 = row.get("max_connections");
    let opened_connections: i32 = row.get("opened_connections");
    Ok((
        StatusCode::OK,
        Json(json!({
            "updated_at": updated_at,
            "dependencies": {
                "database": {
                    "version": version,
                    "max_connections": max_connections,
                    "opened_connections": opened_connections
                }
            }
        })),
    ))
}
