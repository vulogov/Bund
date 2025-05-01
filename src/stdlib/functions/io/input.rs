extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use yansi::Paint;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use yapp::PasswordReader;

pub fn stdlib_bund_prompt_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let prompt = format!("{}{}{}{}{} {} ", Paint::yellow("["), Paint::red("B"), Paint::blue("U").bold(), Paint::white("N"), Paint::cyan("D"), Paint::green(">").bold());
    vm.stack.push(Value::from_string(prompt));
    Ok(vm)
}

pub fn stdlib_input_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline INPUT");
    }

    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => bail!("INPUT returns: {}", err),
    };

    let prompt_value = match vm.stack.pull() {
        Some(prompt_value) => prompt_value,
        None => Value::from_string("> "),
    };

    let prompt = match prompt_value.cast_string() {
        Ok(prompt) => prompt,
        Err(_) => "> ".to_string(),
    };

    let line = rl.readline(&prompt);

    match line {
        Ok(input_line) => {
            vm.stack.push(Value::from_string(input_line.trim()));
        },
        Err(ReadlineError::Interrupted) => {
            log::info!("CTRL-C");
        },
        Err(ReadlineError::Eof) => {
            log::info!("CTRL-D");
        },
        Err(err) => {
            bail!("INPUT line returns: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_input_password_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("PASSWORD error casting message: {}", err),
        },
        None => bail!("PASSWORD: NO DATA #1"),
    };
    let mut passwd_prompt = yapp::Yapp::new().with_echo_symbol('.');
    let res = match passwd_prompt.read_password_with_prompt(&msg) {
        Ok(res) => res,
        Err(err) => bail!("PASSWORD returns: {}", err),
    };
    vm.stack.push(Value::from_string(res));
    Ok(vm)
}

pub fn stdlib_input_loop_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline INPUT");
    }

    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => bail!("INPUT returns: {}", err),
    };

    let lambda_value = match vm.stack.pull() {
        Some(lambda_value) => lambda_value,
        None => bail!("Error getting INPUT* lambda from stack"),
    };

    if ! lambda_value.type_of() == LAMBDA {
        bail!("INPUT*: #1 must be a LAMBDA");
    }

    let prompt_value = match vm.stack.pull() {
        Some(prompt_value) => prompt_value,
        None => Value::from_string("> "),
    };

    let prompt = match prompt_value.cast_string() {
        Ok(prompt) => prompt,
        Err(_) => "> ".to_string(),
    };

    loop {
        let line = rl.readline(&prompt);
        match line {
            Ok(input_line) => {
                vm.stack.push(Value::from_string(input_line.trim()));
                match vm.lambda_eval(lambda_value.clone()) {
                    Ok(_) => {},
                    Err(err) => {
                        bail!("INPUT* returned error from LAMBDA: {}", err);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                log::info!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break;
            },
            Err(err) => {
                bail!("INPUT line returns: {}", err);
            }
        }
    }
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
    let _ = bc.vm.register_inline("input".to_string(), stdlib_input_inline);
    let _ = bc.vm.register_inline("password".to_string(), stdlib_input_password_inline);
    let _ = bc.vm.register_inline("input*".to_string(), stdlib_input_loop_inline);
    let _ = bc.vm.register_inline("bund.prompt".to_string(), stdlib_bund_prompt_inline);
    drop(bc);
}
