extern crate log;
extern crate pest;
use std;
use pest::error::{Error};
use crate::lang::Rule;

pub fn parse_error_handler(err: Error<Rule>) {
    log::trace!("parse_error_handler() reached");
    println!("Error: {:?}", err.variant.message());
    println!("At:    {:?}", err.line());
    // println!("{:?}", err.location.Pos.pos());
}

pub fn exit_at_parse_error_handler(err: Error<Rule>) {
    parse_error_handler(err);
    std::process::exit(0);
}
