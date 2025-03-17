extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::functions::statistics;
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use augurs::{
    ets::AutoETS,
    mstl::MSTLModel,
    prelude::*,
};

fn forecast_mstl_base(vm: &mut VM, op: StackOps, smode: statistics::SourceMode, err_prefix: String) -> Result<&mut VM, Error> {

    let source = match statistics::get_data::get_data(vm, op.clone(), smode.clone(), err_prefix.clone()) {
        Ok(source_val) => source_val,
        Err(err) => bail!("{} NO DATA #2: {}", &err_prefix, err),
    };
    let n = match vm.stack.pull() {
        Some(n_value) => match n_value.cast_int() {
            Ok(n) => n,
            Err(err) => bail!("{} sensitivity casting error: {}", &err_prefix, err),
        },
        None => bail!("{} NO DATA #2", &err_prefix),
    };
    let interval = match vm.stack.pull() {
        Some(int_value) => match int_value.cast_float() {
            Ok(n) => n,
            Err(err) => bail!("{} interval casting error: {}", &err_prefix, err),
        },
        None => bail!("{} NO DATA #3", &err_prefix),
    };
    let periods = vec![2];
    let ets = AutoETS::non_seasonal().into_trend_model();
    let mstl = MSTLModel::new(periods, ets);
    match mstl.fit(&source) {
        Ok(fit) => {
            let forecasts = match fit.predict(n as usize, interval) {
                Ok(forecast) => forecast,
                Err(err) => bail!("{} forecasting returns: {}", &err_prefix, err),
            };
            let mut res = Value::dict();
            let mut f_values = Value::list();
            for i in &forecasts.point {
                f_values = f_values.push(Value::from_float(*i));
            }
            res = res.set("forecast", f_values);
            let mut f_low_values = Value::list();
            let mut f_up_values = Value::list();
            match forecasts.intervals {
                Some(ref intervals) => {
                    res = res.set("interval", Value::from_float(intervals.level));
                    for i in &intervals.lower {
                        f_low_values = f_low_values.push(Value::from_float(*i));
                    }
                    for i in &intervals.upper {
                        f_up_values = f_up_values.push(Value::from_float(*i));
                    }
                }
                None => {}
            }
            res = res.set("lower", f_low_values);
            res = res.set("upper", f_up_values);
            vm.stack.push(res);
        }
        Err(err) => bail!("{} fitting returns: {}", &err_prefix, err),
    };
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_forecast_stack_consume_mstl(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_mstl_base(vm, StackOps::FromStack, statistics::SourceMode::Consume, "FORECAST.MSTL".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_wb_consume_mstl(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_mstl_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Consume, "FORECAST.MSTL.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_stack_keep_mstl(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_mstl_base(vm, StackOps::FromStack, statistics::SourceMode::Keep, "FORECAST.MSTL,".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_wb_keep_mstl(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_mstl_base(vm, StackOps::FromWorkBench, statistics::SourceMode::Keep, "FORECAST.MSTL.,".to_string())
}



pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("forecast.mstl".to_string(), stdlib_forecast_stack_consume_mstl);
    let _ = bc.vm.register_inline("forecast.mstl.".to_string(), stdlib_forecast_wb_consume_mstl);
    let _ = bc.vm.register_inline("forecast.mstl,".to_string(), stdlib_forecast_stack_keep_mstl);
    let _ = bc.vm.register_inline("forecast.mstl.,".to_string(), stdlib_forecast_wb_keep_mstl);

    drop(bc);
}
