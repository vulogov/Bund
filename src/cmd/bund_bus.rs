extern crate log;
use crate::cmd;

#[time_graph::instrument]
pub fn bund_bus_init(c: &cmd::Cli) {
    log::debug!("BUND::bus_init() reached");
    if c.distributed {
        log::debug!("BUND interpreter is running as a node in distributed network");
    }
}
