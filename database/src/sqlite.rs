use std::collections::HashSet;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, Error, Row, Statement, ToSql};
use rusqlite::types::FromSql;

/// Connect to SQLite database
///
/// Note: if the database does not exist, will create it
pub fn connect_to_sqlite_db<P: AsRef<Path> + Clone>(path: P) -> Result<Connection, SqliteConnectionError> where PathBuf: From<P> {
	let pathbuf = PathBuf::from(path.clone());
	let parent = pathbuf.parent();
	if let Some(parent) = parent {
		std::fs::create_dir_all(parent)?;
	}

	let connection = Connection::open(path)?;
	connection.execute("PRAGMA foreign_keys = ON;", [])?;
	Ok(connection)
}

#[derive(thiserror::Error, Debug)]
pub enum SqliteConnectionError {
	#[error("Could not connect to sqlite database. Cause {0}")]
	Database(#[from] Error),
	#[error("Could not create folder where to store database. Cause {0}")]
	CreateFolder(#[from] std::io::Error),
}
