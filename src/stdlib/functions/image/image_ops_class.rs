extern crate log;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{functions};
use easy_error::{Error, bail};


pub fn register_method_image_greyscale(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::GREYSCALE: NO DATA #1"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::GRAYSCALE returns: {}", err),
    };
    let gray_img = img.grayscale();
    let res_rgb = gray_img.to_rgb8();
    let image_data = Value::make_envelope(res_rgb.into_raw());
    let new_value_object = value_object.set("image".to_string(), image_data);
    vm.stack.push(new_value_object);
    Ok(vm)
}

pub fn register_method_image_blur(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::BLUR: NO DATA #1"),
    };
    let scale = match vm.stack.pull() {
        Some(scale) => match scale.cast_float() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::BLUR can not cast scale: {}", err),
        },
        None => bail!("IMAGE::BLUR: NO DATA #2"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::BLUR returns: {}", err),
    };
    let gimg = img.blur(scale as f32);
    let image_data = Value::make_envelope(gimg.into_bytes());
    let new_value_object = value_object.set("image".to_string(), image_data);
    vm.stack.push(new_value_object);
    Ok(vm)
}

pub fn register_method_image_contrast(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::CONTRAST: NO DATA #1"),
    };
    let scale = match vm.stack.pull() {
        Some(scale) => match scale.cast_float() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::CONSTRAST can not cast scale: {}", err),
        },
        None => bail!("IMAGE::CONTRAST: NO DATA #2"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::CONTRAST returns: {}", err),
    };
    let gimg = img.adjust_contrast(scale as f32);
    let image_data = Value::make_envelope(gimg.into_bytes());
    let new_value_object = value_object.set("image".to_string(), image_data);
    vm.stack.push(new_value_object);
    Ok(vm)
}

pub fn register_method_image_brighten(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::BRIGHTEN: NO DATA #1"),
    };
    let scale = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::BRIGHTEN can not cast scale: {}", err),
        },
        None => bail!("IMAGE::BRIGHTEN: NO DATA #2"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::BRIGHTEN returns: {}", err),
    };
    let gimg = img.brighten(scale as i32);
    let image_data = Value::make_envelope(gimg.into_bytes());
    let new_value_object = value_object.set("image".to_string(), image_data);
    vm.stack.push(new_value_object);
    Ok(vm)
}
