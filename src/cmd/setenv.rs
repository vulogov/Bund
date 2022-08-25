extern crate log;
use crate::cmd;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_run(_c: &cmd::Cli) {
    log::trace!("setenv::setenv() reached");

}
