use std::env;
use tokio_postgres::{Config, NoTls, Row, types::ToSql};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("variavel de ambiente: {0}")]
    Env(#[from] env::VarError),
    #[error("porta invalida: {0}")]
    Port(#[from] std::num::ParseIntError),
    #[error("postgres: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}

pub async fn query(sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, DbError> {
    let mut config = Config::new();
    config
        .host(&env::var("POSTGRES_HOST")?)
        .port(env::var("POSTGRES_PORT")?.parse::<u16>()?)
        .user(&env::var("POSTGRES_USER")?)
        .password(&env::var("POSTGRES_PASSWORD")?)
        .dbname(&env::var("POSTGRES_DB")?);
    let (client, connection) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("erro na conexão com o Postgres: {e}")
        }
    });

    let result = client.query(sql, params).await?;
    Ok(result)
}
