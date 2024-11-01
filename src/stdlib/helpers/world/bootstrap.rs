extern crate log;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};


pub fn save_bootstrap<'a>(vm: &'a mut VM, _conn: &mut Connection, _name: String, _snippet: String) -> Result<&'a mut VM, Error> {
    Ok(vm)
}

pub fn init<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS BOOTSTRAP", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping bootstrap table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS BOOTSTRAP (
            name    TEXT PRIMARY KEY,
            script  BLOB
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating bootstrap table returns: {}", err);
        }
    }
    Ok(vm)
}
