use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{bail, Error};
use crate::cmd;
use nanohtml2text;

fn stdlib_convert_html_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let object = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let object_val = match object {
        Some(object_val) => object_val,
        None => {
            bail!("{} returns NO DATA #1", &err_prefix);
        }
    };
    let object_data = match object_val.cast_string() {
        Ok(object_data) => object_data,
        Err(err) => {
            bail!("{} casting data returns: {}", &err_prefix, err);
        }
    };
    let res_str = nanohtml2text::html2text(&object_data);
    let value = Value::from_string(res_str);
    let _ = match op {
        StackOps::FromStack => vm.stack.push(value),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(value),
    };
    return Ok(vm);
}

#[time_graph::instrument]
pub fn stdlib_convert_from_html_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_html_base(vm, StackOps::FromStack, "CONVERT.FROM_HTML".to_string())
}

#[time_graph::instrument]
pub fn stdlib_convert_from_html_workbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_convert_html_base(vm, StackOps::FromWorkBench, "CONVERT.FROM_HTML.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("convert.from_html".to_string(), stdlib_convert_from_html_stack_inline);
    let _ = bc.vm.register_inline("convert.from_html.".to_string(), stdlib_convert_from_html_workbench_inline);
    drop(bc);
}
