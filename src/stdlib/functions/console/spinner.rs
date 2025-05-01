extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::types::*;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use crate::stdlib::Mutex;
use std::ops::DerefMut;
use lazy_static::lazy_static;
use spinoff::{Spinner, spinners};
use rusty_termcolor::colors;

lazy_static! {
    static ref SPINNER: Mutex<Option<Spinner>> = {
        let e: Mutex<Option<Spinner>>  = Mutex::new(None);
        e
    };
}

pub fn stdlib_spinner_start(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.update_text(msg);
        },
        None => {
            *s = Some(Spinner::new(spinners::Dots, msg, None));
        },
    };
    drop(s);
    Ok(vm)
}

pub fn stdlib_spinner_stop(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.stop_with_message(&msg);
            *s = None;
        },
        None => {
        },
    };
    drop(s);
    Ok(vm)
}

pub fn stdlib_spinner_success(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.success(&msg);
            *s = None;
        },
        None => {
        },
    };
    drop(s);
    Ok(vm)
}

pub fn stdlib_spinner_fail(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.fail(&msg);
            *s = None;
        },
        None => {
        },
    };
    drop(s);
    Ok(vm)
}

pub fn stdlib_spinner_warn(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.warn(&msg);
            *s = Some(Spinner::new(spinners::Dots, "continue", None));
        },
        None => {
        },
    };
    drop(s);
    Ok(vm)
}


pub fn stdlib_spinner_info(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(spinner) => {
            spinner.info(&msg);
            *s = Some(Spinner::new(spinners::Dots, "continue", None));
        },
        None => {
        },
    };
    drop(s);
    Ok(vm)
}

pub fn stdlib_spinner_text(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.conv(STRING) {
            Ok(msg) => match msg.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("CONSOLE::SPINNER error casting message: {}", err),
            },
            Err(err) => bail!("CONSOLE::SPINNER error converting message: {}", err),
        },
        None => bail!("CONSOLE::SPINNER: NO DATA #1"),
    };
    let mut s = SPINNER.lock().unwrap();
    match s.deref_mut() {
        Some(_) => {
            println!("");
            rusty_termcolor::println_colored(&msg, &colors::WHITE);
        },
        None => {
            rusty_termcolor::println_colored(&msg, &colors::WHITE);
        },
    };
    drop(s);
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
    let _ = bc.vm.register_inline("console.spinner".to_string(), stdlib_spinner_start);
    let _ = bc.vm.register_inline("console.spinner.stop".to_string(), stdlib_spinner_stop);
    let _ = bc.vm.register_inline("console.spinner.success".to_string(), stdlib_spinner_success);
    let _ = bc.vm.register_inline("console.spinner.fail".to_string(), stdlib_spinner_fail);
    let _ = bc.vm.register_inline("console.spinner.warning".to_string(), stdlib_spinner_warn);
    let _ = bc.vm.register_inline("console.spinner.info".to_string(), stdlib_spinner_info);
    let _ = bc.vm.register_inline("spinner.text".to_string(), stdlib_spinner_text);
    drop(bc);
}
