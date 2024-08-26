#![deny(warnings)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "mssql")]
pub mod mssql;

#[cfg(feature = "oracle")]
pub mod oracle;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;
