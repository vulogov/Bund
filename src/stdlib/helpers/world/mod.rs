use rusqlite::{Connection};
use easy_error::{Error, bail};

pub mod aliases;

#[derive(Debug, Clone)]
pub enum WorldFunctions {
    All,
    Aliases,
}

pub fn open(file_name: String) -> Result<Connection, Error> {
    match Connection::open(file_name) {
        Ok(connection) => {
            return Ok(connection);
        }
        Err(err) => {
            bail!("Open world operation returns: {}", err);
        }
    }
}
