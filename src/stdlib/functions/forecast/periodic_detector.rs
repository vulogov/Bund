extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error;
use easy_error::{bail};
use rust_dynamic::value::Value;
use augurs::seasons::{Detector, PeriodogramDetector};
use crate::stdlib::functions::statistics;


fn seasons_estimate_base(vm: &mut VM, op: StackOps, err_prefix: String) -> std::result::Result<&mut VM, easy_error::Error> {
    let source = match statistics::get_data::get_data(vm, op.clone(), statistics::SourceMode::Consume, err_prefix.clone()) {
        Ok(source_val) => source_val,
        Err(err) => bail!("{} NO DATA #2: {}", &err_prefix, err),
    };

    let periods = PeriodogramDetector::default().detect(&source);
    if periods.len() == 0 {
        let mut res = Value::list();
        for v in source {
            res = res.push(Value::from_float(v));
        }
        vm.stack.push(res);
        return Ok(vm);
    }
    let mut res  = Value::list();
    let mut c = 0;
    'outer: loop {
        for p in &periods {
            if c >= source.len() {
                break 'outer;
            }
            let mut row = Value::list();
            let interval = if c + *p as usize > source.len() {
                c..source.len()
            } else {
                c..(c+*p as usize)
            };
            for v in &source[interval] {
                row = row.push(Value::from_float(*v));
            }
            c = c + *p as usize;
            res = res.push(row);
        }
    }
    vm.stack.push(res);
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_season_estimate_stack(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    seasons_estimate_base(vm, StackOps::FromStack, "PERIODIC.DETECT".to_string())
}
#[time_graph::instrument]
pub fn stdlib_season_estimate_wb(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    seasons_estimate_base(vm, StackOps::FromWorkBench, "PERIODIC.DETECT.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("periodic.detect".to_string(), stdlib_season_estimate_stack);
    let _ = bc.vm.register_inline("periodic.detect.".to_string(), stdlib_season_estimate_wb);

    drop(bc);
}
