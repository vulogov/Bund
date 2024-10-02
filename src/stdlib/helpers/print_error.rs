use easy_error::{Error};
use crate::cmd;

pub fn print_error(err: Error, _cli: &cmd::Cli) {
    println!("{}", err);
}

pub fn print_error_plain(err: Error) {
    println!("{}", err);
}

pub fn print_error_from_str(err: String, _cli: &cmd::Cli) {
    println!("{}", err);
}

pub fn print_error_from_str_plain(err: String) {
    println!("{}", err);
}
