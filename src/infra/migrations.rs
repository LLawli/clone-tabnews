use crate::infra::database::{self, DbError};
use serde_json::{Value, json};

mod embedded {
    refinery::embed_migrations!("infra/migrations");
}

const LOCK_KEY: i64 = 0x6d69_6772;

fn to_json(m: &refinery::Migration) -> Value {
    json!({ "version": m.version(), "name": m.name() })
}

pub async fn pending() -> Result<Vec<Value>, DbError> {
    let mut client = database::connect().await?;
    let runner = embedded::migrations::runner();

    let applied: Vec<_> = runner
        .get_applied_migrations_async(&mut client)
        .await?
        .iter()
        .map(|m| m.version())
        .collect();

    Ok(runner
        .get_migrations()
        .iter()
        .filter(|m| !applied.contains(&m.version()))
        .map(to_json)
        .collect())
}

pub async fn run() -> Result<Vec<Value>, DbError> {
    let mut client = database::connect().await?;

    client
        .execute("SELECT pg_advisory_lock($1)", &[&LOCK_KEY])
        .await?;
    let result = embedded::migrations::runner().run_async(&mut client).await;
    client
        .execute("SELECT pg_advisory_unlock($1)", &[&LOCK_KEY])
        .await?;

    Ok(result?.applied_migrations().iter().map(to_json).collect())
}
