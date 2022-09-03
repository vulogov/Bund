
extern crate pest;
use pest::{Parser};
use pest_derive::*;
use crate::vm::error;

#[derive(Parser)]
#[grammar = "bund.pest"]
struct BUNDParser;

pub mod parse;

pub fn parse(s: &String) {
    let pairs = BUNDParser::parse(Rule::program, s);
    match pairs {
        Ok(_) => {
            for pair in pairs {
                for p in pair {
                    parse::parse_pair(p);
                }
            }
        }
        Err(err) => {
            error::parse_error_handler(err);
        }
    }

}
