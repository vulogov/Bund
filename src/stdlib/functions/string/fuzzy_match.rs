extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;


pub fn string_fuzzy_match_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 2 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let string1_val_get = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let string2_val_get = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull(),
    };

    let string1_val = match string1_val_get {
        Some(string1_val) => string1_val,
        None => {
            bail!("{} returns NO DATA #1", &err_prefix);
        }
    };
    let string2_val = match string2_val_get {
        Some(string2_val) => string2_val,
        None => {
            bail!("{} returns NO DATA #2", &err_prefix);
        }
    };

    let string1_data = match string1_val.cast_string() {
        Ok(string1_data) => string1_data,
        Err(err) => {
            bail!("{} returned for #1: {}", &err_prefix, err);
        }
    };
    let string2_data = match string2_val.cast_string() {
        Ok(string2_data) => string2_data,
        Err(err) => {
            bail!("{} returned for #2: {}", &err_prefix, err);
        }
    };
    let matcher = SkimMatcherV2::default();
    let res = match matcher.fuzzy_match(&string1_data, &string2_data) {
        Some(res) => res as i64,
        None => 0 as i64,
    };
    let _ = match op {
        StackOps::FromStack => vm.stack.push(Value::from_int(res)),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_int(res)),
    };
    Ok(vm)
}


pub fn stdlib_string_stack_fuzzymatch_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_fuzzy_match_base(vm, StackOps::FromStack, "STRING.FUZZYMATCH".to_string())
}

pub fn stdlib_string_workbench_fuzzymatch_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_fuzzy_match_base(vm, StackOps::FromWorkBench, "STRING.FUZZYMATCH.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("string.fuzzymatch".to_string(), stdlib_string_stack_fuzzymatch_inline);
    let _ = bc.vm.register_inline("string.fuzzymatch.".to_string(), stdlib_string_workbench_fuzzymatch_inline);

    drop(bc);
}
