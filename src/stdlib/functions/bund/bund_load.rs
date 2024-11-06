
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use rusqlite::{Connection};
use easy_error::{Error, bail};

fn bund_load<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    log::debug!("Loading aliases");
    match helpers::world::aliases::load_aliases(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases LOAD returns: {}", err)
        }
    }
    log::debug!("Loading lambdas");
    match helpers::world::lambdas::load_lambdas(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Lambdas LOAD returns: {}", err)
        }
    }
    log::debug!("Loading stacks");
    match helpers::world::stacks::load_stacks(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Stacks LOAD returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_load_with_scripts<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match bund_load(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("LOAD returns: {}", err)
        }
    }
    let scripts = match helpers::world::bootstrap::read_all_bootstrap(vm, conn) {
        Ok(scripts) => scripts,
        Err(err) => {
            bail!("BOOTSTRAP discovery scripts returns: {}", err);
        }
    };
    log::debug!("Running bootstrap scripts");
    for snippet in scripts {
        match helpers::eval::bund_compile_and_eval(vm, snippet) {
            Ok(_) => {},
            Err(err) => {
                bail!("BOOTSTRAP execution of script is failed: {}", err);
            }
        }
    }
    Ok(vm)
}

fn bund_load_aliases<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::aliases::load_aliases(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Aliases LOAD returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_load_lambdas<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::lambdas::load_lambdas(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Lambdas LOAD returns: {}", err)
        }
    }
    Ok(vm)
}

fn bund_load_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match helpers::world::stacks::load_stacks(vm, conn) {
        Ok(_) => {}
        Err(err) => {
            bail!("Stacks LOAD returns: {}", err)
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_load_base(vm: &mut VM, op: helpers::world::WorldFunctions) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for LOAD");
    }
    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("LOAD returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("LOAD casting string returns: {}", err);
        }
    };
    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    let res = match op {
        helpers::world::WorldFunctions::All => bund_load(vm, &mut conn),
        helpers::world::WorldFunctions::Aliases => bund_load_aliases(vm, &mut conn),
        helpers::world::WorldFunctions::Lambdas => bund_load_lambdas(vm, &mut conn),
        helpers::world::WorldFunctions::Stacks => bund_load_stacks(vm, &mut conn),
    };
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    return res;
}

pub fn stdlib_bund_bootstrap(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for BOOTSTRAP");
    }
    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("LOAD returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("LOAD casting string returns: {}", err);
        }
    };
    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    let res = bund_load_with_scripts(vm, &mut conn);
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    return res;
}

pub fn stdlib_bund_load(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_load_base(vm, helpers::world::WorldFunctions::All)
}

pub fn stdlib_bund_load_aliases(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_load_base(vm, helpers::world::WorldFunctions::Aliases)
}

pub fn stdlib_bund_load_lambdas(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_load_base(vm, helpers::world::WorldFunctions::Lambdas)
}

pub fn stdlib_bund_load_stacks(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_load_base(vm, helpers::world::WorldFunctions::Stacks)
}

pub fn stdlib_bund_load_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund LOAD functions disabled with --noio");
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
        let _ = bc.vm.register_inline("load".to_string(), stdlib_bund_load_disabled);
        let _ = bc.vm.register_inline("load.aliases".to_string(), stdlib_bund_load_disabled);
        let _ = bc.vm.register_inline("load.lambdas".to_string(), stdlib_bund_load_disabled);
        let _ = bc.vm.register_inline("load.stacks".to_string(), stdlib_bund_load_disabled);
        let _ = bc.vm.register_inline("bootstrap".to_string(), stdlib_bund_load_disabled);
    } else {
        let _ = bc.vm.register_inline("load".to_string(), stdlib_bund_load);
        let _ = bc.vm.register_inline("load.aliases".to_string(), stdlib_bund_load_aliases);
        let _ = bc.vm.register_inline("load.lambdas".to_string(), stdlib_bund_load_lambdas);
        let _ = bc.vm.register_inline("load.stacks".to_string(), stdlib_bund_load_stacks);
        let _ = bc.vm.register_inline("bootstrap".to_string(), stdlib_bund_bootstrap);
    }
    drop(bc);
}
