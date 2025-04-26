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


pub fn register_method_image_crop(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::CROP: NO DATA #1"),
    };
    let x = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::CROP can not cast scale: {}", err),
        },
        None => bail!("IMAGE::CROP: NO DATA #2"),
    };
    let y = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::CROP can not cast scale: {}", err),
        },
        None => bail!("IMAGE::CROP: NO DATA #3"),
    };
    let w = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::CROP can not cast scale: {}", err),
        },
        None => bail!("IMAGE::CROP: NO DATA #4"),
    };
    let h = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::CROP can not cast scale: {}", err),
        },
        None => bail!("IMAGE::CROP: NO DATA #5"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::CROP returns: {}", err),
    };
    let gimg = img.crop_imm(x as u32, y as u32, w as u32, h as u32);
    let image_data = Value::make_envelope(gimg.clone().into_bytes());
    let mut new_value_object = value_object.set("image".to_string(), image_data);
    let mut new_value_object = new_value_object.set("H".to_string(), Value::from_int(gimg.height() as i64));
    let new_value_object = new_value_object.set("W".to_string(), Value::from_int(gimg.width() as i64));
    vm.stack.push(new_value_object);
    Ok(vm)
}

pub fn register_method_image_thumbnail(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::THUMBNAIL: NO DATA #1"),
    };
    let w = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::THUMBNAIL can not cast scale: {}", err),
        },
        None => bail!("IMAGE::THUMBNAIL: NO DATA #2"),
    };
    let h = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::THUMBNAIL can not cast scale: {}", err),
        },
        None => bail!("IMAGE::THUMBNAIL: NO DATA #3"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::THUMBNAIL returns: {}", err),
    };
    let gimg = img.thumbnail_exact(w as u32, h as u32);
    let image_data = Value::make_envelope(gimg.clone().into_bytes());
    let mut new_value_object = value_object.set("image".to_string(), image_data);
    let mut new_value_object = new_value_object.set("H".to_string(), Value::from_int(gimg.height() as i64));
    let new_value_object = new_value_object.set("W".to_string(), Value::from_int(gimg.width() as i64));
    vm.stack.push(new_value_object);
    Ok(vm)
}

pub fn register_method_image_unsharp(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::UNSHARP: NO DATA #1"),
    };
    let thr = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::UNSHARP can not cast threshold: {}", err),
        },
        None => bail!("IMAGE::UNSHARP: NO DATA #2"),
    };
    let sigma = match vm.stack.pull() {
        Some(scale) => match scale.cast_float() {
            Ok(scale) => scale,
            Err(err) => bail!("IMAGE::UNSHARP can not cast sigma: {}", err),
        },
        None => bail!("IMAGE::UNSHARP: NO DATA #3"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::UNSHARP returns: {}", err),
    };
    let gimg = img.unsharpen(sigma as f32, thr as i32);
    let image_data = Value::make_envelope(gimg.clone().into_bytes());
    let mut new_value_object = value_object.set("image".to_string(), image_data);
    let mut new_value_object = new_value_object.set("H".to_string(), Value::from_int(gimg.height() as i64));
    let new_value_object = new_value_object.set("W".to_string(), Value::from_int(gimg.width() as i64));
    vm.stack.push(new_value_object);
    Ok(vm)
}
