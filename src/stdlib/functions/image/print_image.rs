extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use std::path::Path;
use viuer;


pub fn stdlib_image_print_image_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for IMAGE.PRINT");
    }
    let name = match vm.stack.pull() {
        Some(name_val) => match name_val.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("IMAGE.PRINT error casting file name: {}", err),
        },
        None => bail!("IMAGE.PRINT: NO DATA #1"),
    };
    if ! Path::new(&name).exists() {
        bail!("IMAGE.PRINT: file {} not exists", &name);
    }
    let conf = viuer::Config {
        ..Default::default()
    };
    println!("");
    match viuer::print_from_file(name, &conf) {
        Ok(_) => {},
        Err(err) => bail!("IMAGE.PRINT returns: {}", err),
    };
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
    let _ = bc.vm.register_inline("image.print".to_string(), stdlib_image_print_image_inline);
    drop(bc);
}
