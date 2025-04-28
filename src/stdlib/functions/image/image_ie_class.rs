extern crate log;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{functions};
use easy_error::{Error, bail};


pub fn register_method_image_export(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::EXPORT: NO DATA #1"),
    };
    let res = match functions::image::image_class::exported_image(value_object) {
        Ok(res) => res,
        Err(err) => bail!("IMAGE::EXPORT returns: {}", err),
    };
    vm.stack.push(res);
    Ok(vm)
}
