use std::{env, sync::OnceLock};
use tokio_postgres::{
    Config, Row,
    config::{ChannelBinding, SslMode},
    types::ToSql,
};
use tokio_postgres_tls::MakeRustlsConnect;

fn tls() -> MakeRustlsConnect {
    static TLS: OnceLock<MakeRustlsConnect> = OnceLock::new();
    TLS.get_or_init(|| {
        let roots = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        let cfg = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        MakeRustlsConnect::new(cfg)
    })
    .clone()
}

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
        .dbname(&env::var("POSTGRES_DB")?)
        .ssl_mode(if &env::var("ENV")? == "development" {
            SslMode::Prefer
        } else {
            SslMode::Require
        })
        .channel_binding(if &env::var("ENV")? == "development" {
            ChannelBinding::Prefer
        } else {
            ChannelBinding::Require
        });
    let (client, connection) = config.connect(tls()).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("erro na conexão com o Postgres: {e}")
        }
    });

    let result = client.query(sql, params).await?;
    std::mem::forget(client);
    Ok(result)
}
