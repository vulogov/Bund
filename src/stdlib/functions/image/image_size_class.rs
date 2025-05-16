extern crate log;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use fast_image_resize::{IntoImageView, Resizer, ResizeOptions, ResizeAlg, FilterType};
use fast_image_resize::images::Image;
use crate::stdlib::{functions};
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn register_method_image_upscale(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::UPSCALE: NO DATA #1"),
    };
    let scale = match vm.stack.pull() {
        Some(scale) => match scale.cast_int() {
            Ok(scale) => scale as u32,
            Err(err) => bail!("IMAGE::UPSCALE can not cast scale: {}", err),
        },
        None => bail!("IMAGE::UPSCALE: NO DATA #2"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::UPSCALE returns: {}", err),
    };

    let width = img.width();
    let height = img.height();
    let new_width = width * scale;
    let new_height = height * scale;

    log::debug!("Resizing image W={}->{} H={}->{}", &width, &new_width, &height, &new_height);

    let pixel_type = match img.pixel_type() {
        Some(pixel_type) => pixel_type,
        None => bail!("IMAGE::UPSCALE can not detect pixel type"),
    };

    let mut dst_image = Image::new(
        new_width,
        new_height,
        pixel_type,
    );

    let mut resizer = Resizer::new();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        resizer.set_cpu_extensions(fast_image_resize::CpuExtensions::Sse4_1);
    }
    match resizer.resize(
        &img,
        &mut dst_image,
        &ResizeOptions::new()
            .resize_alg(ResizeAlg::Convolution(FilterType::Mitchell)),
    ) {
        Ok(_) => {},
        Err(err) => bail!("IMAGE::UPSCALE returns: {}", err),
    };

    let image_data = Value::make_envelope(dst_image.into_vec());
    let mut new_value_object = value_object.set("image".to_string(), image_data);
    let mut new_value_object = new_value_object.set("H".to_string(), Value::from_int(new_height as i64));
    let new_value_object = new_value_object.set("W".to_string(), Value::from_int(new_width as i64));
    vm.stack.push(new_value_object);
    Ok(vm)
}
