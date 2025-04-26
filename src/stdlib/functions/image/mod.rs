extern crate log;
use crate::cmd;

pub mod print_image;
pub mod image_class;
pub mod image_ops_class;
pub mod image_face_class;
pub mod image_ie_class;
pub mod image_size_class;

pub fn init_stdlib(cli: &cmd::Cli) {
    print_image::init_stdlib(cli);
    image_class::init_stdlib(cli);
}
