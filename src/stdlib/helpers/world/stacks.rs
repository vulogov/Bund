extern crate log;
// use rust_dynamic::value::Value;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {

    Ok(vm)
}

pub fn save_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS STACKS", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping stacks table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS STACKS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating stacks table returns: {}", err);
        }
    }
    for n in vm.stack.stacks.iter() {
        match conn.execute("INSERT INTO STACKS (name) VALUES (?1)", (n.clone(),)) {
            Ok(_) => {},
            Err(err) => {
                bail!("Saving stacks returns: {}", err);
            }
        }
    }
    Ok(vm)
}
