extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::functions::statistics;
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use decorum::{R64};
use markov_chain::Chain;

fn forecast_markov_base(vm: &mut VM, op: StackOps, smode: statistics::SourceMode, err_prefix: String) -> Result<&mut VM, Error> {
    match statistics::get_data::get_data(vm, op.clone(), smode, err_prefix.clone()) {
        Ok(source) => {
            let mut dst: Vec<R64> = Vec::new();
            for v in source {
                dst.push(v.into());
            }
            let mut palanteer = Chain::<R64>::new(16);
            palanteer.train(dst);
            let res = palanteer.generate_limit(1);
            if res.len() == 0 {
                bail!("{} forecasting does not returned a prognisis", &err_prefix);
            }
            for i in res {
                let _ = match op {
                    StackOps::FromStack => vm.stack.push(Value::from_float(f64::from(i))),
                    StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_float(f64::from(i))),
                };
            }
        }
        Err(err) => {
            bail!("{} returned: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}

pub fn stdlib_forecast_stack_consume_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromStack, statistics::SourceMode::Consume, "FORECAST.MARKOV".to_string())
}

pub fn stdlib_forecast_wb_consume_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Consume, "FORECAST.MARKOV.".to_string())
}

pub fn stdlib_forecast_stack_keep_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromStack, statistics::SourceMode::Keep, "FORECAST.MARKOV,".to_string())
}

pub fn stdlib_forecast_wb_keep_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Keep, "FORECAST.MARKOV.,".to_string())
}



pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("forecast.markov".to_string(), stdlib_forecast_stack_consume_markov);
    let _ = bc.vm.register_inline("forecast.markov.".to_string(), stdlib_forecast_wb_consume_markov);
    let _ = bc.vm.register_inline("forecast.markov,".to_string(), stdlib_forecast_stack_keep_markov);
    let _ = bc.vm.register_inline("forecast.markov.,".to_string(), stdlib_forecast_wb_keep_markov);

    drop(bc);
}
