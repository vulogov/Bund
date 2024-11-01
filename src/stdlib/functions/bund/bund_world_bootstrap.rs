
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use rusqlite::{Connection};
use easy_error::{Error, bail};


pub fn bund_save_bootstrap_inline<'a>(vm: &'a mut VM, conn: &mut Connection, name: String, snippet: String) -> Result<&'a mut VM, Error> {
    match helpers::world::bootstrap::save_bootstrap(vm, conn, name, snippet) {
        Ok(_) => {}
        Err(err) => {
            bail!("Bootstrap SAVE returns: {}", err)
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_save_bootstrap(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for SAVE.BOOTSTRAP");
    }
    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("SAVE returns NO DATA #1");
        }
    };
    let snippet_value = match vm.stack.pull() {
        Some(snippet_value) => snippet_value,
        None => {
            bail!("SAVE returns NO DATA #2");
        }
    };
    let name_value = match vm.stack.pull() {
        Some(name_value) => name_value,
        None => {
            bail!("SAVE returns NO DATA #3");
        }
    };

    let snippet = match snippet_value.cast_string() {
        Ok(snippet) => snippet,
        Err(err) => {
            bail!("SAVE.BOOTSTRAP casting string #2 returns: {}", err);
        }
    };
    let name = match name_value.cast_string() {
        Ok(name) => name,
        Err(err) => {
            bail!("SAVE.BOOTSTRAP casting string #3 returns: {}", err);
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("SAVE.BOOTSTRAP casting string #1 returns: {}", err);
        }
    };
    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    let res = bund_save_bootstrap_inline(vm, &mut conn, name, snippet);
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    return res;
}

pub fn stdlib_bund_save_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SAVE.BOOTSTRAP functions disabled with --noio");
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.noio {
        let _ = bc.vm.register_inline("save.bootstrap".to_string(), stdlib_bund_save_disabled);
    } else {
        let _ = bc.vm.register_inline("save.bootstrap".to_string(), stdlib_bund_save_bootstrap);
    }
    drop(bc);
}
