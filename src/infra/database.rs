use rustls::pki_types::{CertificateDer, pem::PemObject};
use std::{env, sync::OnceLock};
use tokio_postgres::{
    Client, Config, Row,
    config::{ChannelBinding, SslMode},
    types::ToSql,
};
use tokio_postgres_rustls::MakeRustlsConnect;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("variavel de ambiente: {0}")]
    Env(#[from] env::VarError),
    #[error("porta invalida: {0}")]
    Port(#[from] std::num::ParseIntError),
    #[error("postgres: {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("certificado CA invalido: {0}")]
    Pem(#[from] rustls::pki_types::pem::Error),
    #[error("CA rejeitado pelo rustls: {0}")]
    Rustls(#[from] rustls::Error),
    #[error("POSTGRES_CA definido mas sem nenhum certificado")]
    EmptyCa,
    #[error("migration: {0}")]
    Migration(#[from] refinery::Error),
}

fn build_tls() -> Result<MakeRustlsConnect, DbError> {
    let mut roots = rustls::RootCertStore::empty();
    match env::var("POSTGRES_CA") {
        Ok(pem) => {
            let pem = pem.replace("\\n", "\n");
            let mut n = 0;
            for cert in CertificateDer::pem_slice_iter(pem.as_bytes()) {
                roots.add(cert?)?;
                n += 1;
            }
            if n == 0 {
                return Err(DbError::EmptyCa);
            }
        }
        Err(env::VarError::NotPresent) => {
            roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        }
        Err(e) => return Err(e.into()),
    }

    let cfg = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(MakeRustlsConnect::new(cfg))
}

fn tls() -> Result<MakeRustlsConnect, DbError> {
    static TLS: OnceLock<MakeRustlsConnect> = OnceLock::new();
    if let Some(t) = TLS.get() {
        return Ok(t.clone());
    }
    let t = build_tls()?;
    Ok(TLS.get_or_init(|| t).clone())
}

pub async fn connect() -> Result<Client, DbError> {
    let dev = env::var("ENV").is_ok_and(|v| v == "development");
    let mut config = Config::new();
    config
        .host(&env::var("POSTGRES_HOST")?)
        .port(env::var("POSTGRES_PORT")?.parse::<u16>()?)
        .user(&env::var("POSTGRES_USER")?)
        .password(&env::var("POSTGRES_PASSWORD")?)
        .dbname(&env::var("POSTGRES_DB")?)
        .ssl_mode(if dev {
            SslMode::Prefer
        } else {
            SslMode::Require
        })
        .channel_binding(if dev {
            ChannelBinding::Prefer
        } else {
            ChannelBinding::Require
        });

    let (client, connection) = config.connect(tls()?).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in Postgres connection: {}", e)
        }
    });
    Ok(client)
}

pub async fn query(sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, DbError> {
    let client = connect().await?;
    Ok(client.query(sql, params).await?)
}
