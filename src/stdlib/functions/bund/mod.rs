extern crate log;
use crate::cmd;

pub mod bund_eval;
pub mod bund_use;
pub mod bund_args;
pub mod bund_load;
pub mod bund_save;
pub mod bund_models;
pub mod bund_world_bootstrap;
pub mod bund_exit;
pub mod bund_fun;
pub mod bund_class;
pub mod bund_interpreter;

pub fn init_stdlib(cli: &cmd::Cli) {
    log::debug!("Initializing BUND: module");
    bund_eval::init_stdlib(cli);
    bund_use::init_stdlib(cli);
    bund_load::init_stdlib(cli);
    bund_save::init_stdlib(cli);
    bund_models::init_stdlib(cli);
    bund_args::init_stdlib(cli);
    bund_world_bootstrap::init_stdlib(cli);
    bund_exit::init_stdlib(cli);
    bund_fun::init_stdlib(cli);
    bund_class::init_stdlib(cli);
    bund_interpreter::init_stdlib(cli);
}
