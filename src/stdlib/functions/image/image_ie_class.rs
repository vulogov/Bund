extern crate log;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{functions};
use easy_error::{Error, bail};


pub fn register_method_image_export(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::FACEDETECT: NO DATA #1"),
    };
    let _img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::FACEDETECT returns: {}", err),
    };
    Ok(vm)
}
