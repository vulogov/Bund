extern crate log;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn stdlib_conditional_raise(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg_val) => {
            let msg = match msg_val.cast_string() {
                Ok(msg) => msg,
                Err(_) => "Can not cast a message for raise".to_string(),
            };
            msg
        },
        None => "No message for raise".to_string(),
    };
    bail!("{}", msg)
}
