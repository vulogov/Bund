extern crate log;
// use rust_dynamic::value::Value;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {

    Ok(vm)
}

fn save_stack_data<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS STACK_DATA", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping stack_data table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS STACK_DATA (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            pos     INTEGER,
            value   BLOB
        )",
        ()) {
        Ok(_) => {
            match conn.execute(
                "DROP INDEX IF EXISTS STACK_DATA.STACK_AND_POS",
            ()) {
                Ok(_) => {
                    match conn.execute(
                        "CREATE UNIQUE INDEX IF NOT EXISTS STACK_AND_POS ON STACK_DATA(name,pos)",
                    ()) {
                        Ok(_) => {},
                        Err(err) => {
                            bail!("Creating stack_data index table returns: {}", err);
                        }
                    }

                }
                Err(err) => {
                    bail!("Dropping stack_data index table returns: {}", err);
                }
            }
        },
        Err(err) => {
            bail!("Creating stack_data table returns: {}", err);
        }
    }
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
    save_stack_data(vm, conn)
}
