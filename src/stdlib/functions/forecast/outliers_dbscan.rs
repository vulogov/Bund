extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error;
use easy_error::{bail};
use crate::stdlib::functions::statistics;
use crate::stdlib::functions::math;


fn outliers_dbscan_estimate_base(vm: &mut VM, op: StackOps, err_prefix: String) -> std::result::Result<&mut VM, easy_error::Error> {
    let sensitivity_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let sensitivity = match sensitivity_value {
        Some(sensitivity_value) => match sensitivity_value.cast_float() {
            Ok(sensitivity) => sensitivity,
            Err(err) => bail!("{} sensitivity casting error: {}", &err_prefix, err),
        },
        None => bail!("{} NO DATA #1", &err_prefix),
    };
    let source1 = match statistics::get_data::get_data(vm, op.clone(), statistics::SourceMode::Consume, err_prefix.clone()) {
        Ok(source_val) => source_val,
        Err(err) => bail!("{} NO DATA #2: {}", &err_prefix, err),
    };
    let source2 = match statistics::get_data::get_data(vm, op.clone(), statistics::SourceMode::Consume, err_prefix.clone()) {
        Ok(source_val) => source_val,
        Err(err) => bail!("{} NO DATA #2: {}", &err_prefix, err),
    };
    let outliers = match math::anomalies::detect_outliers_dbscan(source1, source2, sensitivity) {
        Ok(outliers) => outliers,
        Err(err) => bail!("{} return error: {}", &err_prefix, err),
    };
    vm.stack.push(outliers);
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_outlier_dbscan_estimate_stack(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    outliers_dbscan_estimate_base(vm, StackOps::FromStack, "OUTLIER.DETECT".to_string())
}
#[time_graph::instrument]
pub fn stdlib_outlier_dbscan_estimate_wb(vm: &mut VM) -> std::result::Result<&mut VM, easy_error::Error> {
    outliers_dbscan_estimate_base(vm, StackOps::FromWorkBench, "OUTLIER.DETECT.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("outlier.detect.dbscan".to_string(), stdlib_outlier_dbscan_estimate_stack);
    let _ = bc.vm.register_inline("outlier.detect.dbscan.".to_string(), stdlib_outlier_dbscan_estimate_wb);

    drop(bc);
}
