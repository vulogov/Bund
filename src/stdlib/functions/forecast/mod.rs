extern crate log;
use crate::cmd;

pub mod markov;
pub mod mstl;
pub mod estimation;
pub mod outliers;
pub mod outliers_dbscan;
pub mod clustering;

pub fn init_stdlib(cli: &cmd::Cli) {
    markov::init_stdlib(cli);
    mstl::init_stdlib(cli);
    estimation::init_stdlib(cli);
    outliers::init_stdlib(cli);
    outliers_dbscan::init_stdlib(cli);
    clustering::init_stdlib(cli);
}
