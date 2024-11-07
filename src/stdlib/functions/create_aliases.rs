use crate::stdlib::BUND;
use crate::cmd;
use crate::stdlib::helpers;

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_alias("!!".to_string(), "bund.eval".to_string());
    let _ = bc.vm.register_alias("cp".to_string(), "fs.cp".to_string());
    let _ = bc.vm.register_alias("mv".to_string(), "fs.mv".to_string());
    let _ = bc.vm.register_alias("rm".to_string(), "fs.rm".to_string());
    drop(bc);
}
