extern crate log;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

#[derive(Debug)]
pub struct Alias {
    pub id: i32,
    pub name: String,
    pub alias: String,
}

pub fn load_aliases<'a>(vm: &'a mut VM, _conn: &mut Connection) -> Result<&'a mut VM, Error> {
    Ok(vm)
}

pub fn save_aliases<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE ALIASES", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping aliases table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE ALIASES (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            alias   TEXT NOT NULL
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating aliases table returns: {}", err);
        }
    }
    Ok(vm)
}
