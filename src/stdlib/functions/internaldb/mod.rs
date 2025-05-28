extern crate log;
use crate::cmd;
use crate::stdlib::helpers;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::stdlib::BUND;
use easy_error::{Error, bail};
use duckdb::{Connection, Config};

pub mod sql;
pub mod execute;
pub mod prql;
pub mod document_store;

lazy_static! {
    pub static ref DB: Mutex<Connection> = {
        let cli = cmd::CLI.lock().unwrap();
        let conf = Config::default()
                   .enable_autoload_extension(false).expect("INTERNALDB: Configuration expected");
        let conn = match &cli.internal_db {
            Some(name) => {
                log::debug!("Internal database is in {}", &name);
                Connection::open_with_flags(name, conf).expect("Conenction to in-file internal database is expected")
            }
            None =>  {
                log::debug!("Creating in-memory internal database");
                Connection::open_in_memory_with_flags(conf).expect("Conenction to in-memory internal database is expected")
            }
        };
        let e: Mutex<Connection> = Mutex::new(conn);
        drop(cli);
        e
    };
}

#[time_graph::instrument]
pub fn stdlib_internaldb_version(vm: &mut VM) -> Result<&mut VM, Error> {
    let db = match DB.lock() {
        Ok(db) => db,
        Err(err) => {
            bail!("Can not access DB: {}", err);
        }
    };
    let res = match db.version() {
        Ok(version) => Value::from_string(version),
        Err(_) => Value::from_string("unknown"),
    };
    drop(db);
    vm.stack.push(res);
    Ok(vm)
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    lazy_static::initialize(&DB);
    let _ = bc.vm.register_inline("internaldb.version".to_string(), stdlib_internaldb_version);
    let _ = bc.vm.register_inline("internaldb.sql".to_string(), sql::stdlib_internaldb_sql);
    let _ = bc.vm.register_inline("internaldb.execute".to_string(), execute::stdlib_internaldb_execute);
    let _ = bc.vm.register_inline("internaldb.prql".to_string(), prql::stdlib_internal_prql);
    let _ = bc.vm.register_inline("documentstore.init".to_string(), document_store::stdlib_internaldb_docstore_init);
    let _ = bc.vm.register_inline("documentstore.drop".to_string(), document_store::stdlib_internaldb_docstore_drop);
    drop(bc);
}
