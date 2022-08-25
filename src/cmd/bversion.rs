extern crate log;
use neofiglet::FIGfont;
use crate::cmd;

pub fn run_version(_c: &cmd::Cli) {
    log::trace!("run_version() reached");
    let standard_font = FIGfont::standard().unwrap();
    let banner = format!("the BUND {}", env!("CARGO_PKG_VERSION"));
    let figure = standard_font.convert(&banner);
    println!("{}", figure.unwrap());
}
