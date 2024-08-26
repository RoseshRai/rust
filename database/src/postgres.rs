use std::future::Future;
use std::pin::Pin;
use std::num::ParseIntError;
use std::str::FromStr;
use deadpool_postgres::{Config, CreatePoolError, Pool, PoolConfig, RecyclingMethod, Runtime};
use tokio_postgres::{Client, Error, NoTls};
use thiserror::Error;

pub enum DatabaseKind {
    NoSSL,
    SSL,
}

#[derive(Error, Debug)]
pub enum PostgresConnectionError {
    #[error("Environment variable {0} should be set")]
    Configuration(String),
    #[error("Could not connect to database. Cause {0}")]
    Database(#[from] Error),
    #[error("Could not connect to database. Cause {0}")]
    PoolError(#[from] CreatePoolError),
    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] ParseIntError),
}

pub async fn async_connect_to_postgres(database_kind: &DatabaseKind, database_user: &str, database_password: &str, database_host: &str, database_port: &str, database_name: &str) -> Result<Client, PostgresConnectionError> {
    let connection_string = match database_kind {
        DatabaseKind::NoSSL => format!("user={} host={} port={} password={} dbname={}", database_user, database_host, database_port, database_password, database_name),
        DatabaseKind::SSL => format!("user={} host={} port={} password={} dbname={} sslmode=require", database_user, database_host, database_port, database_password, database_name),
    };

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;
    tokio::spawn(async move { connection.await.expect("Connection error"); });
    Ok(client)
}

pub fn connect_to_postgres(database_kind: &DatabaseKind, database_user: &str, database_password: &str, database_host: &str, database_port: &str, database_name: &str) -> Result<postgres::Client, PostgresConnectionError> {
    let connection_string = match database_kind {
        DatabaseKind::NoSSL => format!("user={} host={} port={} password={} dbname={}", database_user, database_host, database_port, database_password, database_name),
        DatabaseKind::SSL => format!("user={} host={} port={} password={} dbname={} sslmode=require", database_user, database_host, database_port, database_password, database_name),
    };

    let client = postgres::Client::connect(&connection_string, NoTls)?;
    Ok(client)
}

const MAX_SIZE_POOL: usize = 4;

pub async fn create_pool(database_kind: &DatabaseKind, database_user: &str, database_password: &str, database_host: &str, database_port: &str, database_name: &str) -> Result<Pool, PostgresConnectionError> {
    let mut config = Config::new();
    config.user = Some(database_user.to_string());
    config.password = Some(database_password.to_string());
    config.host = Some(database_host.to_string());
    config.port = Some(u16::from_str(database_port)?);
    config.dbname = Some(database_name.to_string());
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::default(),
    });
    config.pool = Some(PoolConfig::new(MAX_SIZE_POOL));

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}
