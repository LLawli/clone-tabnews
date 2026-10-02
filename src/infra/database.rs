use rustls::pki_types::{CertificateDer, pem::PemObject};
use std::{env, sync::OnceLock};
use tokio_postgres::{
    Config, Row,
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
    let (client, connection) = match config.connect(tls()?).await {
        Ok(client_connection) => client_connection,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Err(DbError::from(e));
        }
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("erro na conexão com o Postgres: {e}")
        }
    });

    let result = client.query(sql, params).await?;
    Ok(result)
}
