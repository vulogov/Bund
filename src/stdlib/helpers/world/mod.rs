use sanitize_filename;
use rusqlite::{Connection};
use easy_error::{Error, bail};

pub mod aliases;
pub mod lambdas;
pub mod stacks;

#[derive(Debug, Clone)]
pub enum WorldFunctions {
    All,
    Aliases,
    Lambdas,
    Stacks,
}

pub fn open(file_name: String) -> Result<Connection, Error> {
    let options = sanitize_filename::Options {
    truncate: true,
    windows: true,
    replacement: ""
    };
    let sanitized_filename = sanitize_filename::sanitize_with_options(file_name, options);
    let world_file_name = format!("{}.world", sanitized_filename);
    match Connection::open(world_file_name) {
        Ok(connection) => {
            return Ok(connection);
        }
        Err(err) => {
            bail!("Open world operation returns: {}", err);
        }
    }
}
