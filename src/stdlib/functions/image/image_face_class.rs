extern crate log;
use rust_multistackvm::multistackvm::VM;
use rust_dynamic::value::Value;
use crate::stdlib::{functions};
use rustface::{ImageData};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use easy_error::{Error, bail};


pub fn register_method_image_facedetect(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::FACEDETECT: NO DATA #1"),
    };
    let img = match functions::image::image_class::make_image(value_object.clone()) {
        Ok(img) => img,
        Err(err) => bail!("IMAGE::FACEDETECT returns: {}", err),
    };
    let gray_img = img.grayscale();
    let mut res_rgb = gray_img.to_rgb8();
    let gray_data = gray_img.clone().into_bytes();
    let mut image_data = ImageData::new(
       &gray_data,
       gray_img.width() as u32,
       gray_img.height() as u32);
    let mut detector = match rustface::create_detector("/Users/gandalf/Src/Bund/Models/facedetection/seeta_fd_frontal_v1.0.bin") {
        Ok(detector) => detector,
        Err(err) => bail!("IMAGE::FACEDETECT error creating detector: {}", err),
    };
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);
    for face in detector.detect(&mut image_data).into_iter() {
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
        draw_hollow_rect_mut(&mut res_rgb, rect, image::Rgb([255, 0, 0]));
    }
    let image_envelope = Value::make_envelope(res_rgb.into_raw());
    let new_value_object = value_object.set("image".to_string(), image_envelope);
    vm.stack.push(new_value_object);
    Ok(vm)
}
