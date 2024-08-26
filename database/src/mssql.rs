use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;

/// Connect to MSSQL database
pub async fn connect_to_mssql(database_user: &str, database_password: &str, database_host: &str, database_port: u16, database_name: &str) -> Result<Client<Compat<TcpStream>>, MssqlConnectionError> {
	let mut config = Config::new();

	config.host(database_host);
	config.port(database_port);
	config.database(database_name);
	config.authentication(AuthMethod::sql_server(database_user, database_password));
	config.trust_cert();

	let tcp = TcpStream::connect(config.get_addr()).await?;
	tcp.set_nodelay(true)?;

	let client = Client::connect(config, tcp.compat_write()).await?;

	Ok(client)
}

#[derive(thiserror::Error, Debug)]
pub enum MssqlConnectionError {
	#[error("Could not connect to mssql database. Cause {0}")]
	Database(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for MssqlConnectionError {
	fn from(error: std::io::Error) -> Self {
		MssqlConnectionError::Database(Box::new(error))
	}
}

impl From<tiberius::error::Error> for MssqlConnectionError {
	fn from(error: tiberius::error::Error) -> Self {
		MssqlConnectionError::Database(Box::new(error))
	}
}