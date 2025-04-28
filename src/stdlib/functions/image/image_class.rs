extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};
use std::path::Path;
use image;

pub fn exported_image(value: Value) -> Result<Value, Error> {
    let h: u32 = match value.get("H") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let w: u32 = match value.get("W") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let pixels: Vec<u8> = match value.get("image") {
        Ok(pixels) => match pixels.cast_bin() {
            Ok(pixels) => pixels,
            Err(err) => bail!("{}", err),
        }
        Err(err) => bail!("{}", err),
    };
    let data = Value::make_envelope(pixels);
    let mut res = functions::values::exported_wrap(
        functions::values::IMAGE,
        data
    );
    res = res.set("H", Value::from_int(h as i64));
    res = res.set("W", Value::from_int(w as i64));
    Ok(res)
}

pub fn exported_from_dynamic(img: image::DynamicImage) -> Result<Value, Error> {
    let data = Value::make_envelope(img.clone().into_bytes());
    let mut res = functions::values::exported_wrap(
        functions::values::IMAGE,
        data
    );
    res = res.set("H", Value::from_int(img.height() as i64));
    res = res.set("W", Value::from_int(img.width() as i64));
    Ok(res)
}

pub fn exported_to_dynamic(value: Value) -> Result<image::DynamicImage, Error> {
    match functions::values::exported_type_of(value.clone()) {
        Some(functions::values::IMAGE) => {},
        _ => bail!("Exported package having a wrong type"),
    };
    let h: u32 = match value.get("H") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let w: u32 = match value.get("W") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let pixels = match value.get(".data") {
        Ok(pixels) => match pixels.cast_bin() {
            Ok(pixels) => pixels,
            Err(err) => bail!("{}", err),
        }
        Err(err) => bail!("{}", err),
    };
    let img: image::RgbImage = match image::ImageBuffer::from_raw(w, h, pixels) {
        Some(img) => img,
        None => bail!("Can not create an image"),
    };
    Ok(image::DynamicImage::ImageRgb8(img))
}

fn image_load(mut value: Value, name: String) -> Result<Value, Error> {
    if ! Path::new(&name).exists() {
        bail!("IMAGE.LOAD: file {} not exists", &name);
    }
    let image_val = match image::open(name) {
        Ok(di) => di,
        Err(err) => bail!("IMAGE::LOAD returns {}", err),
    };
    let image_data = Value::make_envelope(image_val.clone().into_bytes());
    let mut new_value_object = value.set("image".to_string(), image_data);
    let mut new_value_object = new_value_object.set("H".to_string(), Value::from_int(image_val.height() as i64));
    let new_value_object = new_value_object.set("W".to_string(), Value::from_int(image_val.width() as i64));
    Ok(new_value_object)
}

pub fn make_image(value: Value) -> Result<image::DynamicImage, Error> {
    let h: u32 = match value.get("H") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let w: u32 = match value.get("W") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let pixels: Vec<u8> = match value.get("image") {
        Ok(pixels) => match pixels.cast_bin() {
            Ok(pixels) => pixels,
            Err(err) => bail!("{}", err),
        }
        Err(err) => bail!("{}", err),
    };
    let img: image::RgbImage = match image::ImageBuffer::from_raw(w, h, pixels) {
        Some(img) => img,
        None => bail!("Can not create an image"),
    };
    Ok(image::DynamicImage::ImageRgb8(img))
}

pub fn make_grayscale_image(value: Value) -> Result<image::DynamicImage, Error> {
    let h: u32 = match value.get("H") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let w: u32 = match value.get("W") {
        Ok(h) => match h.cast_int() {
            Ok(h) => h as u32,
            Err(err) => bail!("{}", err),
        },
        Err(err) => bail!("{}", err),
    };
    let pixels: Vec<u8> = match value.get("image") {
        Ok(pixels) => match pixels.cast_bin() {
            Ok(pixels) => pixels,
            Err(err) => bail!("{}", err),
        }
        Err(err) => bail!("{}", err),
    };
    let img: image::GrayImage = match image::ImageBuffer::from_raw(w, h, pixels) {
        Some(img) => img,
        None => bail!("Can not create an image"),
    };
    Ok(image::DynamicImage::ImageLuma8(img))
}

fn register_method_image_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                MAP => {
                    match functions::values::exported_type_of(data_object.clone()) {
                        Some(functions::values::IMAGE) => {},
                        _ => bail!("Exported package having a wrong type"),
                    };
                    let mut new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(""));
                    let mut new_value_object = new_value_object.set("image", data_object.get(".data").unwrap());
                    let mut new_value_object = new_value_object.set("W", data_object.get("W").unwrap());
                    let new_value_object = new_value_object.set("H", data_object.get("H").unwrap());
                    vm.stack.push(new_value_object);
                }
                STRING => {
                    let data = data_object.cast_string().unwrap();
                    if ! Path::new(&data).exists() {
                        bail!("IMAGE.INIT: file {} not exists", &data);
                    }
                    let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&data));
                    let new_value_object_w_data = match image_load(new_value_object, data) {
                        Ok(new_value_object) => new_value_object,
                        Err(err) => bail!("{}", err),
                    };
                    vm.stack.push(new_value_object_w_data);
                }
                _ => bail!("IMAGE: WRAPPED TYPE IS NOT SUPPORTED: {}", &data_object.type_name()),
            }
        }
        None => bail!("IMAGE: NO WRAPPED DATA WAS FOUND FOR INIT"),
    }
    Ok(vm)
}



fn register_method_image_print(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for method 'IMAGE::PRINT'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::PRINT: NO DATA #1"),
    };
    match value_object.get("image".to_string()) {
        Ok(image_object) => {
            if image_object.type_of() == ENVELOPE {
                let img = match make_image(value_object.clone()) {
                    Ok(img) => img,
                    Err(err) => bail!("IMAGE::PRINT returns: {}", err),
                };
                let conf = viuer::Config {
                    ..Default::default()
                };
                match viuer::print(&img, &conf) {
                    Ok(_) => {},
                    Err(err) => bail!("IMAGE.PRINT prints returns: {}", err),
                };
            } else {
                bail!("IMAGE::PRINT data object is not ENVELOPE");
            }
            vm.stack.push(value_object);
        }
        Err(err) => bail!("IMAGE: NO WRAPPED DATA WAS FOUND FOR IMAGE::PRINT {}", err),
    }
    Ok(vm)
}

fn register_method_image_save(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for method 'IMAGE::SAVE'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::SAVE: NO DATA #1"),
    };
    let fname = match vm.stack.pull() {
        Some(fname_object) => match fname_object.cast_string() {
            Ok(fname) => fname,
            Err(err) => bail!("IMAGE::SAVE casting file name returns: {}", err),
        },
        None => bail!("IMAGE::SAVE: NO DATA #2"),
    };
    log::debug!("IMAGE::SAVE to {}", &fname);
    match value_object.get("image".to_string()) {
        Ok(image_object) => {
            if image_object.type_of() == ENVELOPE {
                let img = match make_image(value_object.clone()) {
                    Ok(img) => img,
                    Err(err) => bail!("IMAGE::SAVE returns: {}", err),
                };
                let _ = match img.save(fname) {
                    Ok(()) => {},
                    Err(err) => bail!("IMAGE::SAVE returns {}", err),
                };
            } else {
                bail!("IMAGE::SAVE data object is not ENVELOPE");
            }
            vm.stack.push(value_object);
        }
        Err(err) => bail!("IMAGE: NO WRAPPED DATA WAS FOUND FOR IMAGE::SAVE {}", err),
    }
    Ok(vm)
}

fn register_method_image_save_gray(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for method 'IMAGE::SAVE'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::SAVE: NO DATA #1"),
    };
    let fname = match vm.stack.pull() {
        Some(fname_object) => match fname_object.cast_string() {
            Ok(fname) => fname,
            Err(err) => bail!("IMAGE::SAVE casting file name returns: {}", err),
        },
        None => bail!("IMAGE::SAVE: NO DATA #2"),
    };
    match value_object.get("image".to_string()) {
        Ok(image_object) => {
            if image_object.type_of() == ENVELOPE {
                let img = match make_grayscale_image(value_object.clone()) {
                    Ok(img) => img,
                    Err(err) => bail!("IMAGE::SAVE returns: {}", err),
                };
                let _ = match img.save(fname) {
                    Ok(()) => {},
                    Err(err) => bail!("IMAGE::SAVE returns {}", err),
                };
            } else {
                bail!("IMAGE::SAVE data object is not ENVELOPE");
            }
            vm.stack.push(value_object);
        }
        Err(err) => bail!("IMAGE: NO WRAPPED DATA WAS FOUND FOR IMAGE::SAVE {}", err),
    }
    Ok(vm)
}

fn register_image(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".image_init".to_string(), register_method_image_init);
    let _ = vm.register_method(".image_print".to_string(), register_method_image_print);
    let _ = vm.register_method(".image_save".to_string(), register_method_image_save);
    let _ = vm.register_method(".image_save_grayscale".to_string(), register_method_image_save_gray);
    let _ = vm.register_method(".image_export".to_string(), functions::image::image_ie_class::register_method_image_export);
    let _ = vm.register_method(".image_grayscale".to_string(), functions::image::image_ops_class::register_method_image_greyscale);
    let _ = vm.register_method(".image_blur".to_string(), functions::image::image_ops_class::register_method_image_blur);
    let _ = vm.register_method(".image_brighten".to_string(), functions::image::image_ops_class::register_method_image_brighten);
    let _ = vm.register_method(".image_contrast".to_string(), functions::image::image_ops_class::register_method_image_contrast);
    let _ = vm.register_method(".image_crop".to_string(), functions::image::image_ops_class::register_method_image_crop);
    let _ = vm.register_method(".image_thumbnail".to_string(), functions::image::image_ops_class::register_method_image_thumbnail);
    let _ = vm.register_method(".image_unsharp".to_string(), functions::image::image_ops_class::register_method_image_unsharp);
    let _ = vm.register_method(".image_facedetect".to_string(), functions::image::image_face_class::register_method_image_facedetect);
    let _ = vm.register_method(".image_upscale".to_string(), functions::image::image_size_class::register_method_image_upscale);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("IMAGE"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".image_init".to_string(), Vec::new()));
    obj_class = obj_class.set("print".to_string(), Value::ptr(".image_print".to_string(), Vec::new()));
    obj_class = obj_class.set("save".to_string(), Value::ptr(".image_save".to_string(), Vec::new()));
    obj_class = obj_class.set("save.grayscale".to_string(), Value::ptr(".image_save_grayscale".to_string(), Vec::new()));
    obj_class = obj_class.set("grayscale".to_string(), Value::ptr(".image_grayscale".to_string(), Vec::new()));
    obj_class = obj_class.set("export".to_string(), Value::ptr(".image_export".to_string(), Vec::new()));
    obj_class = obj_class.set("blur".to_string(), Value::ptr(".image_blur".to_string(), Vec::new()));
    obj_class = obj_class.set("brighten".to_string(), Value::ptr(".image_brighten".to_string(), Vec::new()));
    obj_class = obj_class.set("contrast".to_string(), Value::ptr(".image_contrast".to_string(), Vec::new()));
    obj_class = obj_class.set("crop".to_string(), Value::ptr(".image_crop".to_string(), Vec::new()));
    obj_class = obj_class.set("thumbnail".to_string(), Value::ptr(".image_thumbnail".to_string(), Vec::new()));
    obj_class = obj_class.set("unsharp".to_string(), Value::ptr(".image_unsharp".to_string(), Vec::new()));
    obj_class = obj_class.set("facedetect".to_string(), Value::ptr(".image_facedetect".to_string(), Vec::new()));
    obj_class = obj_class.set("upscale".to_string(), Value::ptr(".image_upscale".to_string(), Vec::new()));
    vm.register_class("IMAGE".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    //
    // Register Value
    //
    match register_image(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing IMAGE base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
