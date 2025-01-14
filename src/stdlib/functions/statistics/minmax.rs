extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::functions::statistics;
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum MinMaxOperation {
    Min,
    Max,
}

fn stats_minmax_base(vm: &mut VM, op: StackOps, smode: statistics::SourceMode, mmop: MinMaxOperation, err_prefix: String) -> Result<&mut VM, Error> {
    match statistics::get_data::get_data(vm, op.clone(), smode, err_prefix.clone()) {
        Ok(source) => {
            let res = match mmop {
                MinMaxOperation::Min => source.iter().cloned().fold(0./0., f64::min),
                MinMaxOperation::Max => source.iter().cloned().fold(0./0., f64::max),
            };
            let _ = match op {
                StackOps::FromStack => vm.stack.push(Value::from_float(res as f64)),
                StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_float(res as f64)),
            };
        }
        Err(err) => {
            bail!("{} returned: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_math_minmax_stack_consume_min(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromStack, statistics::SourceMode::Consume, MinMaxOperation::Min, "MATH.MIN".to_string())
}
#[time_graph::instrument]
pub fn stdlib_math_minmax_stack_consume_max(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromStack, statistics::SourceMode::Consume, MinMaxOperation::Max, "MATH.MAX".to_string())
}

#[time_graph::instrument]
pub fn stdlib_math_minmax_workbench_consume_min(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Consume, MinMaxOperation::Min, "MATH.MIN.".to_string())
}
#[time_graph::instrument]
pub fn stdlib_math_minmax_workbench_consume_max(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Consume, MinMaxOperation::Max, "MATH.MAX.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_math_minmax_stack_keep_min(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromStack, statistics::SourceMode::Keep, MinMaxOperation::Min, "MATH.MIN,".to_string())
}
#[time_graph::instrument]
pub fn stdlib_math_minmax_stack_keep_max(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromStack, statistics::SourceMode::Keep, MinMaxOperation::Max, "MATH.MAX,".to_string())
}

#[time_graph::instrument]
pub fn stdlib_math_minmax_workbench_keep_min(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Keep, MinMaxOperation::Min, "MATH.MIN.,".to_string())
}
#[time_graph::instrument]
pub fn stdlib_math_minmax_workbench_keep_max(vm: &mut VM) -> Result<&mut VM, Error> {
    stats_minmax_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Keep, MinMaxOperation::Max, "MATH.MAX.,".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("math.min".to_string(), stdlib_math_minmax_stack_consume_min);
    let _ = bc.vm.register_inline("math.max".to_string(), stdlib_math_minmax_stack_consume_max);
    let _ = bc.vm.register_inline("math.min.".to_string(), stdlib_math_minmax_workbench_consume_min);
    let _ = bc.vm.register_inline("math.max.".to_string(), stdlib_math_minmax_workbench_consume_max);
    let _ = bc.vm.register_inline("math.min,".to_string(), stdlib_math_minmax_stack_keep_min);
    let _ = bc.vm.register_inline("math.max,".to_string(), stdlib_math_minmax_stack_keep_max);
    let _ = bc.vm.register_inline("math.min.,".to_string(), stdlib_math_minmax_workbench_keep_min);
    let _ = bc.vm.register_inline("math.max.,".to_string(), stdlib_math_minmax_workbench_keep_max);

    drop(bc);
}
