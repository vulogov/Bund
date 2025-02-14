extern crate log;
use crate::stdlib::functions::internaldb;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_internaldb_execute(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline INTERNALDB.EXECUTE");
    }
    let query_val = match vm.stack.pull() {
        Some(query_val) => query_val,
        None => bail!("INTERNALDB.EXECUTE: NO DATA #1"),
    };
    let query_str = match query_val.cast_string() {
        Ok(query_str) => query_str,
        Err(err) => bail!("INTERNALDB.EXECUTE: casting query returns error: {}", err),
    };
    let db = match internaldb::DB.lock() {
        Ok(db) => db,
        Err(err) => bail!("INTERNALDB.EXECUTE: getting reference to internal database returns error: {}", err),
    };
    log::debug!("INTERNALDB.EXECUTE: preparing query for internaldb.sql: {}", &query_str);
    let mut stmt = match db.prepare(&query_str) {
        Ok(stmt) => stmt,
        Err(err) => {
            drop(db);
            bail!("INTERNALDB.EXECUTE: prepare statement returns error: {}", err)
        },
    };
    log::debug!("INTERNALDB.EXECUTE: executing statement");
    match stmt.raw_execute() {
        Ok(res) => {
            log::debug!("INTERNALDB.EXECUTE returns: {}", res);
        }
        Err(err) => {
            bail!("INTERNALDB.EXECUTE returned an error: {}", err);
        }
    }
    drop(db);
    Ok(vm)
}
