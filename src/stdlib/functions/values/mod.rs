extern crate log;
use crate::cmd;

use rust_dynamic::types::*;
use rust_dynamic::value::Value;

pub mod make_call_value;
pub mod sort_lists;
pub mod push;
pub mod pull;
pub mod merge;
pub mod unfold;
pub mod getsetinplace;
pub mod listop;

pub const NONE:     i64         = 0;
pub const VALUE:    i64         = 1;
pub const STACK:    i64         = 2;
pub const IMAGE:    i64         = 30;

pub fn exported_type_of(value: Value) -> Option<i64> {
    match value.type_of() {
        MAP => {
            match value.get("type") {
                Ok(val) => match val.cast_int() {
                    Ok(t) => return Some(t),
                    Err(_) => return None,
                }
                Err(_) => return None,
            }
        }
        _ => return None,
    }
}

pub fn exported_wrap(t: i64, value: Value) -> Value {
    let mut res = Value::dict();
    res = res.set("type", Value::from_int(t));
    res = res.set(".data", value.clone());
    res
}

pub fn exported_unwrap(value: Value) -> Option<Value> {
    match value.type_of() {
        MAP => {
            match value.get(".data") {
                Ok(val) => return Some(val),
                Err(_) => return None,
            }
        }
        _ => return None,
    }
}


pub fn init_stdlib(cli: &cmd::Cli) {
    make_call_value::init_stdlib(cli);
    sort_lists::init_stdlib(cli);
    push::init_stdlib(cli);
    pull::init_stdlib(cli);
    unfold::init_stdlib(cli);
    getsetinplace::init_stdlib(cli);
    merge::init_stdlib(cli);
    listop::init_stdlib(cli);
}
