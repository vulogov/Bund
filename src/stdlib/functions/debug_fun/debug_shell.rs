extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use yansi::Paint;

pub fn debug_shell(vm: &mut VM)  -> Result<&mut VM, Error> {
    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => {
            helpers::print_error::print_error_from_str_plain(format!("{}", err));
            return Ok(vm);
        }
    };
    // #[cfg(feature = "with-file-history")]
    if rl.load_history("bund_debug_shell_history.txt").is_err() {
        log::warn!("No previous shell history.");
    }
    let prompt = format!("{}{} {} ", Paint::yellow("["), Paint::red("DEBUG"), Paint::white(">").bold());
    loop {
        let line = rl.readline(&prompt);
        match line {
            Ok(snippet) => {
                let _ = rl.add_history_entry(snippet.as_str());
                match helpers::eval::bund_compile_and_eval(vm, snippet) {
                    Ok(_) => continue,
                    Err(err) => {
                        helpers::print_error::print_error_from_str_plain(format!("{}", err));
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                log::info!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break
            },
            Err(err) => {
                helpers::print_error::print_error_from_str_plain(format!("{}", err));
            }
        }
    }
    log::debug!("Saving shell history...");
    // #[cfg(feature = "with-file-history")]
    let _ = rl.save_history("bund_debug_shell_history.txt");
    Ok(vm)
}

pub fn stdlib_debug_shell(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_shell(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("debug.shell".to_string(), stdlib_debug_shell);
    drop(bc);
}
