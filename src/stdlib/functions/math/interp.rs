use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::functions::statistics;
use interp::{interp, InterpMode};
use easy_error::{Error, bail};
use crate::cmd;


#[time_graph::instrument]
pub fn stdlib_math_interp_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for MATH.INTERPOLATION");
    }
    let x = match statistics::get_data::get_data(vm,
        StackOps::FromStack,
        statistics::SourceMode::Consume, "MATH.INTERPOLATION".to_string()) {
        Ok(x) => x,
        Err(err) => bail!("MATH.INTERPOLATION getting X returns: {}", err),
    };
    let y = match statistics::get_data::get_data(vm,
        StackOps::FromStack,
        statistics::SourceMode::Consume, "MATH.INTERPOLATION".to_string()) {
        Ok(y) => y,
        Err(err) => bail!("MATH.INTERPOLATION getting Y returns: {}", err),
    };
    let xp = match vm.stack.pull() {
        Some(xp) => match xp.cast_float() {
            Ok(xp) => xp,
            Err(err) => bail!("MATH.INTERPOLATION error casting XP: {}", err),
        },
        None => bail!("MATH.INTERPOLATION: NO DATA #3"),
    };
    let res: f64 = interp(&x, &y, xp, &InterpMode::default());
    vm.stack.push(Value::from_float(res));
    Ok(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("math.interpolation".to_string(), stdlib_math_interp_inline);
}
