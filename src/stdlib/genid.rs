extern crate log;
use uuid::{Uuid};
use hostname;

pub fn generate_id() -> String {
    log::trace!("generate_id() reached");
    generate_generate().simple().encode_lower(&mut Uuid::encode_buffer()).to_string()
}

pub fn generate_host_id() -> String {
    log::trace!("generate_host_id() reached");
    let name = hostname::get().unwrap();
    format!("{}@{}", name.to_string_lossy(), generate_generate().simple().encode_lower(&mut Uuid::encode_buffer()).to_string())
}

pub fn generate_generate() -> Uuid {
    Uuid::new_v4()
}
