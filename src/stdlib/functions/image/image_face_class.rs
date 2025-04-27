extern crate log;
use rust_multistackvm::multistackvm::VM;
use rust_dynamic::value::Value;
use crate::stdlib::{functions};
use rustface::{ImageData};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use easy_error::{Error, bail};
use std::path::Path;


pub fn register_method_image_facedetect(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("IMAGE::FACEDETECT: NO DATA #1"),
    };
    let model_path = match vm.stack.pull() {
        Some(model_path) => match model_path.cast_string() {
            Ok(model_path) => model_path,
            Err(err) => bail!("IMAGE::FACEDETECT error casting model path: {}", err),
        },
        None => bail!("IMAGE::FACEDETECT: NO DATA #2"),
    };
    if ! Path::new(&model_path).exists() {
        bail!("IMAGE.FACEDETECT: file with model {} not exists", &model_path);
    }
    log::debug!("IMAGE::FACEDETECT model at: {}", &model_path);
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
    let mut detector = match rustface::create_detector(&model_path) {
        Ok(detector) => detector,
        Err(err) => bail!("IMAGE::FACEDETECT error creating detector: {}", err),
    };
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);
    let mut res = Value::list();
    for face in detector.detect(&mut image_data).into_iter() {
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
        draw_hollow_rect_mut(&mut res_rgb, rect, image::Rgb([255, 0, 0]));
        let some_face = img.crop_imm(bbox.x() as u32, bbox.y() as u32, bbox.width() as u32, bbox.height() as u32);
        let some_face_value = functions::image::image_class::exported_from_dynamic(some_face).unwrap();
        res = res.push(some_face_value);
    }
    log::debug!("IMAGE::FACEDETECT {} faces detected", res.len());
    let image_envelope = Value::make_envelope(res_rgb.into_raw());
    let mut new_value_object = value_object.set("image".to_string(), image_envelope);
    let new_value_object = new_value_object.set("faces".to_string(), res);
    vm.stack.push(new_value_object);
    Ok(vm)
}
