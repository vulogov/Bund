extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_conditional_tryexcept(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("tryexcept"));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    let try_lambda = match value.get("try") {
        Ok(try_lambda) => try_lambda,
        Err(_) => Value::lambda(),
    };
    let except_lambda = match value.get("except") {
        Ok(except_lambda) => except_lambda,
        Err(_) => Value::lambda(),
    };
    let associated_lambda = match value.get("associated") {
        Ok(associated_lambda) => associated_lambda,
        Err(_) => Value::lambda(),
    };
    let recovery_lambda = match value.get("recovery") {
        Ok(recovery_lambda) => recovery_lambda,
        Err(_) => Value::lambda(),
    };
    match vm.lambda_eval(try_lambda) {
        Ok(_) => {},
        Err(err) => {
            log::debug!("TRY lambda returned {}", &err.ctx);
            let mut err_res: Value = Value::conditional();
            err_res = err_res.set("type", Value::from_string("error"));
            err_res = err_res.set("associated", associated_lambda);
            err_res = err_res.set("context", Value::from_string(err.ctx));
            vm.stack.push(err_res);
            match vm.lambda_eval(except_lambda) {
                Ok(_) => {},
                Err(err) => bail!("TRYEXCEPT EXCEPT lambda returns: {}", err),
            };
            match vm.lambda_eval(recovery_lambda) {
                Ok(_) => {},
                Err(err) => bail!("TRYEXCEPT RECOVERY lambda returns: {}", err),
            };

        },
    };
    Ok(vm)
}
