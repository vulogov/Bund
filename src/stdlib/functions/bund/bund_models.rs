
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_bund_load_model(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for LOAD.MODEL");
    }

    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("LOAD.MODEL returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("LOAD.MODEL casting string returns: {}", err);
        }
    };

    let model_name_value = match vm.stack.pull() {
        Some(model_name_value) => model_name_value,
        None => {
            bail!("LOAD.MODEL returns NO DATA #2");
        }
    };
    let model_name = match model_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("LOAD.MODEL casting string returns: {}", err);
        }
    };

    let model_name = model_name.trim().to_string();
    let file_name = file_name.trim().to_string();

    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    match helpers::world::models::load_model(vm, &mut conn, model_name) {
        Ok(_) => {},
        Err(err) => bail!("LOAD.MODEL returns: {}", err),
    };
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_bund_save_model(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for SAVE.MODEL");
    }

    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("SAVE.MODEL returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("SAVE.MODEL casting string returns: {}", err);
        }
    };

    let model_value = match vm.stack.pull() {
        Some(model_value) => model_value,
        None => {
            bail!("SAVE.MODEL returns NO DATA #2");
        }
    };

    let model_name_value = match vm.stack.pull() {
        Some(model_name_value) => model_name_value,
        None => {
            bail!("SAVE.MODEL returns NO DATA #2");
        }
    };
    let model_name = match model_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("SAVE.MODEL casting string returns: {}", err);
        }
    };

    let model_name = model_name.trim().to_string();
    let file_name = file_name.trim().to_string();

    let mut conn = match helpers::world::open(file_name) {
        Ok(conn) => conn,
        Err(err) => {
            bail!("{}", err);
        }
    };
    match helpers::world::models::save_model(vm, &mut conn, model_name, model_value) {
        Ok(_) => {},
        Err(err) => bail!("SAVE.MODEL returns: {}", err),
    };
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_save_model_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SAVE.MODEL functions disabled with --noio");
}

pub fn stdlib_bund_load_model_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SAVE.MODEL functions disabled with --noio");
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
        let _ = bc.vm.register_inline("save.model".to_string(), stdlib_bund_save_model_disabled);
        let _ = bc.vm.register_inline("load.model".to_string(), stdlib_bund_load_model_disabled);
    } else {
        let _ = bc.vm.register_inline("save.model".to_string(), stdlib_bund_save_model);
        let _ = bc.vm.register_inline("load.model".to_string(), stdlib_bund_load_model);
    }

    drop(bc);
}
