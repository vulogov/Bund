extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error;
use easy_error::{bail};
use rstats::*;
use distimate::prelude::*;
use distimate::Pert;
use crate::stdlib::functions::statistics;
use crate::stdlib::functions::math;

#[derive(Debug, Clone)]
pub enum EOperation {
    Estimate,
    Uncertainty,
    Analysis,
}

fn forecast_estimate_base(vm: &mut VM, op: StackOps, eop: EOperation, err_prefix: String) -> std::result::Result<&mut VM, easy_error::Error> {
    match statistics::get_data::get_data(vm, op.clone(), statistics::SourceMode::Consume, err_prefix.clone()) {
        Ok(source) => {
            let shape_value = match op {
                StackOps::FromStack => vm.stack.pull(),
                StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
            };
            let shape_val = match shape_value {
                Some(shape_val) => shape_val,
                None => bail!("{} can not obtain shape", &err_prefix),
            };
            let shape = match shape_val.cast_float() {
                Ok(shape) => shape,
                Err(err) => bail!("{} casting shape returns: {}", &err_prefix, err),
            };
            let min_value = source.iter().cloned().fold(0./0., f64::min);
            let max_value = source.iter().cloned().fold(0./0., f64::max);
            let amean_value = match source.clone().amean() {
                Ok(amean_value) => amean_value,
                Err(err) => bail!("{}: for min={}, max={}, AMEAN error: {}", &err_prefix, min_value, max_value, err),
            };
            let pert = match Pert::new_with_shape(min_value, amean_value, max_value, shape) {
                Ok(pert) => pert,
                Err(err) => bail!("{}: for min={}, max={}, mean={} PERT error: {}", &err_prefix, min_value, max_value, amean_value, err),
            };
            match eop {
                EOperation::Estimate => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(Value::from_float(pert.expected_value())),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_float(pert.expected_value())),
                    };
                }
                EOperation::Uncertainty => {
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(Value::from_float(pert.uncertainty())),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_float(pert.uncertainty())),
                    };
                }
                EOperation::Analysis => {
                    let mut res = Value::dict();
                    res = res.set("expected_value", Value::from_float(pert.expected_value()));
                    res = res.set("uncertainty", Value::from_float(pert.uncertainty()));
                    res = res.set("likely", Value::from_float(pert.most_likely_estimate()));
                    res = res.set("optimistic", Value::from_float(pert.optimistic_estimate()));
                    res = res.set("pessimistic", Value::from_float(pert.pessimistic_estimate()));
                    res = res.set("alpha", Value::from_float(pert.alpha()));
                    res = res.set("beta", Value::from_float(pert.beta()));
                    res = res.set("kurtosis", Value::from_float(pert.kurtosis()));
                    res = res.set("min", Value::from_float(min_value));
                    res = res.set("max", Value::from_float(max_value));
                    res = res.set("mean.arithmetic", Value::from_float(source.amean().unwrap()));
                    res = res.set("mean.geometric", Value::from_float(source.gmean().unwrap()));
                    res = res.set("mean.harmonic", Value::from_float(source.hmean().unwrap()));
                    let (min_1, max_1) = pert.confidence_interval(0.1);
                    let (min_2, max_2) = pert.confidence_interval(0.5);
                    let (min_3, max_3) = pert.confidence_interval(0.99);
                    match pert.calculate_within_interval(&source, 0.2, 0.8) {
                        Ok(data_assurance) => {
                            res = res.set("assurance", Value::from_float(data_assurance));
                        }
                        Err(_) => {}
                    }
                    res = res.set("confidence1", Value::from_list(vec![Value::from_float(min_1), Value::from_float(max_1)]));
                    res = res.set("confidence2", Value::from_list(vec![Value::from_float(min_2), Value::from_float(max_2)]));
                    res = res.set("confidence3", Value::from_list(vec![Value::from_float(min_3), Value::from_float(max_3)]));
                    res = res.set("not_likely_to_overrun", Value::from_float(pert.risk_of_overrun(0.2)));
                    res = res.set("can_overrun", Value::from_float(pert.risk_of_overrun(0.5)));
                    res = res.set("will_overrun", Value::from_float(pert.risk_of_overrun(0.8)));
                    match math::anomalies::detect_anomalies(source.clone(), source.len()/4 as usize) {
                        Ok(anomalies) => {
                            let mut anomalies_values: Vec<Value> = Vec::new();
                            for i in anomalies {
                                anomalies_values.push(Value::from_float(i));
                            }
                            res = res.set("anomalies", Value::from_list(anomalies_values));
                        }
                        Err(err) => bail!("{}", err),
                    }
                    match math::breakout::detect_breakouts(source.clone(), source.len()/4 as usize) {
                        Ok(breakouts) => {
                            let mut breakout_values: Vec<Value> = Vec::new();
                            for i in breakouts {
                                breakout_values.push(Value::from_float(i));
                            }
                            res = res.set("breakouts", Value::from_list(breakout_values));
                        }
                        Err(err) => bail!("{}", err),
                    }
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(res),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
                    };
                }
            }
        }
        Err(err) => {
            bail!("{} returned: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_forecast_estimate_stack(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromStack, EOperation::Estimate, "FORECAST.ESTIMATE".to_string())
}
#[time_graph::instrument]
pub fn stdlib_forecast_estimate_wb(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromWorkBench, EOperation::Estimate, "FORECAST.ESTIMATE.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_estimate_uncertainty_stack(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromStack, EOperation::Uncertainty, "FORECAST.UNCERTAINTY".to_string())
}
#[time_graph::instrument]
pub fn stdlib_forecast_estimate_uncertainty_wb(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromWorkBench, EOperation::Uncertainty, "FORECAST.UNCERTAINTY.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_estimate_analysis_stack(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromStack, EOperation::Analysis, "FORECAST.ESTIMATE!".to_string())
}
#[time_graph::instrument]
pub fn stdlib_forecast_estimate_analysis_wb(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    forecast_estimate_base(vm, StackOps::FromWorkBench, EOperation::Analysis, "FORECAST.ESTIMATE!.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("forecast.estimate".to_string(), stdlib_forecast_estimate_stack);
    let _ = bc.vm.register_inline("forecast.estimate.".to_string(), stdlib_forecast_estimate_wb);
    let _ = bc.vm.register_inline("forecast.estimate.uncertainty".to_string(), stdlib_forecast_estimate_uncertainty_stack);
    let _ = bc.vm.register_inline("forecast.estimate.uncertainty.".to_string(), stdlib_forecast_estimate_uncertainty_wb);
    let _ = bc.vm.register_inline("forecast.estimate!".to_string(), stdlib_forecast_estimate_analysis_stack);
    let _ = bc.vm.register_inline("forecast.estimate!.".to_string(), stdlib_forecast_estimate_analysis_wb);

    drop(bc);
}
