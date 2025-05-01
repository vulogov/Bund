extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


pub fn stdlib_terminal_clear(vm: &mut VM) -> Result<&mut VM, Error> {
    rusty_termcolor::system::clear_screen();
    Ok(vm)
}

pub fn stdlib_terminal_title(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::TITLE error casting message: {}", err),
        },
        None => bail!("CONSOLE::TITLE: NO DATA #1"),
    };
    rusty_termcolor::system::set_title(&msg);
    Ok(vm)
}

pub fn stdlib_terminal_typewriter(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::TYPEWRITER error casting message: {}", err),
        },
        None => bail!("CONSOLE::TYPEWRITER: NO DATA #1"),
    };
    rusty_termcolor::effects::typewriter(&msg, &rusty_termcolor::EffectSettings::default(), None);
    Ok(vm)
}

pub fn stdlib_terminal_box(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("CONSOLE::BOX error casting message: {}", err),
        },
        None => bail!("CONSOLE::BOX: NO DATA #1"),
    };
    let res = rusty_termcolor::formatting::box_text(&msg);
    vm.stack.push(Value::from_string(&res));
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
    let _ = bc.vm.register_inline("console.clear".to_string(), stdlib_terminal_clear);
    let _ = bc.vm.register_inline("console.title".to_string(), stdlib_terminal_title);
    let _ = bc.vm.register_inline("console.typewriter".to_string(), stdlib_terminal_typewriter);
    let _ = bc.vm.register_inline("console.box".to_string(), stdlib_terminal_box);
    drop(bc);
}
