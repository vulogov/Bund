extern crate log;
use bundcore::bundcore::Bund;

pub mod helpers;

pub fn init_stdlib(_bc: &mut Bund) {
    log::debug!("Running STDLIB init");
}
