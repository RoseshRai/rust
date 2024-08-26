use oracle::{Connection, Error};

/// Connect to Oracle database
pub fn connect_to_oracle(database_user: &str, database_password: &str, database_host: &str, database_port: u16, service_name: &str) -> Result<Connection, OracleConnectionError> {
	let connection_string = format!("{}:{}/{}", database_host, database_port, service_name);
	let connection = Connection::connect(database_user, database_password, connection_string)?;

	Ok(connection)
}

#[derive(thiserror::Error, Debug)]
pub enum OracleConnectionError {
	#[error("Could not connect to oracle database. Cause {0}")]
	Database(#[from] Error),
}
