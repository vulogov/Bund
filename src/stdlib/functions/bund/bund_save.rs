
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use rusqlite::{Connection};
use easy_error::{Error, bail};

fn bund_save<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    log::debug!("Saving aliases");
    match helpers::world::aliases::save_aliases(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases SAVE returns: {}", err)
        }
    }
    log::debug!("Saving lambdas");
    match helpers::world::lambdas::save_lambdas(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases SAVE returns: {}", err)
        }
    }
    log::debug!("Saving stacks");
    match helpers::world::stacks::save_stacks(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases SAVE returns: {}", err)
        }
    }
    log::debug!("Priming WORLD for bootstrap scripts");
    match helpers::world::bootstrap::init(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Priming bootstrap returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_save_aliases<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::aliases::save_aliases(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases SAVE returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_save_lambdas<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::lambdas::save_lambdas(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Lambdas SAVE returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_save_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::stacks::save_stacks(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Stacks SAVE returns: {}", err)
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_save_base(vm: &mut VM, op: helpers::world::WorldFunctions) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for SAVE");
    }
    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("SAVE returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("SAVE casting string returns: {}", err);
        }
    };
    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    let res = match op {
        helpers::world::WorldFunctions::All => bund_save(vm, &mut conn),
        helpers::world::WorldFunctions::Aliases => bund_save_aliases(vm, &mut conn),
        helpers::world::WorldFunctions::Lambdas => bund_save_lambdas(vm, &mut conn),
        helpers::world::WorldFunctions::Stacks => bund_save_stacks(vm, &mut conn),
    };
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    return res;
}

#[time_graph::instrument]
pub fn stdlib_bund_save(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_save_base(vm, helpers::world::WorldFunctions::All)
}

#[time_graph::instrument]
pub fn stdlib_bund_save_aliases(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_save_base(vm, helpers::world::WorldFunctions::Aliases)
}

#[time_graph::instrument]
pub fn stdlib_bund_save_lambdas(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_save_base(vm, helpers::world::WorldFunctions::Lambdas)
}

#[time_graph::instrument]
pub fn stdlib_bund_save_stacks(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_save_base(vm, helpers::world::WorldFunctions::Stacks)
}

#[time_graph::instrument]
pub fn stdlib_bund_save_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SAVE functions disabled with --noio");
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
        let _ = bc.vm.register_inline("save".to_string(), stdlib_bund_save_disabled);
        let _ = bc.vm.register_inline("save.aliases".to_string(), stdlib_bund_save_disabled);
        let _ = bc.vm.register_inline("save.lambdas".to_string(), stdlib_bund_save_disabled);
        let _ = bc.vm.register_inline("save.stacks".to_string(), stdlib_bund_save_disabled);
    } else {
        let _ = bc.vm.register_inline("save".to_string(), stdlib_bund_save);
        let _ = bc.vm.register_inline("save.aliases".to_string(), stdlib_bund_save_aliases);
        let _ = bc.vm.register_inline("save.lambdas".to_string(), stdlib_bund_save_lambdas);
        let _ = bc.vm.register_inline("save.stacks".to_string(), stdlib_bund_save_stacks);
    }
    drop(bc);
}
