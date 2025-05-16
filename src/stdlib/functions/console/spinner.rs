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

fn stdlib_spinner_text_base(vm: &mut VM, color: colors::Color) -> Result<&mut VM, Error> {
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
            rusty_termcolor::println_colored(&msg, &color);
        },
        None => {
            rusty_termcolor::println_colored(&msg, &color);
        },
    };
    drop(s);
    Ok(vm)
}

fn stdlib_console_text_base(vm: &mut VM, color: colors::Color) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.conv(STRING) {
            Ok(msg) => match msg.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("CONSOLE error casting message: {}", err),
            },
            Err(err) => bail!("CONSOLE error converting message: {}", err),
        },
        None => bail!("CONSOLE: NO DATA #1"),
    };

    rusty_termcolor::println_colored(&msg, &color);

    Ok(vm)
}

fn stdlib_console_text_fade_base(vm: &mut VM, color1: colors::Color, color2: colors::Color) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.conv(STRING) {
            Ok(msg) => match msg.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("CONSOLE error casting message: {}", err),
            },
            Err(err) => bail!("CONSOLE error converting message: {}", err),
        },
        None => bail!("CONSOLE: NO DATA #1"),
    };
    rusty_termcolor::formatting::print_fade(
        &msg,
        &rusty_termcolor::colors::fade_color(&color1, &color2, msg.len())
    );
    println!("{}", colors::RESET);
    Ok(vm)
}

fn stdlib_console_text_matrix_base(vm: &mut VM, color: colors::Color) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.conv(STRING) {
            Ok(msg) => match msg.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("CONSOLE error casting message: {}", err),
            },
            Err(err) => bail!("CONSOLE error converting message: {}", err),
        },
        None => bail!("CONSOLE: NO DATA #1"),
    };

    rusty_termcolor::effects::matrix_effect(
        &msg,
        &rusty_termcolor::effects::EffectSettings::default(),
        Some(&color)
    );
    print!("{}", colors::RESET);
    Ok(vm)
}

fn stdlib_console_text_rainbow(vm: &mut VM) -> Result<&mut VM, Error> {
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.conv(STRING) {
            Ok(msg) => match msg.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("CONSOLE error casting message: {}", err),
            },
            Err(err) => bail!("CONSOLE error converting message: {}", err),
        },
        None => bail!("CONSOLE: NO DATA #1"),
    };

    rusty_termcolor::effects::rainbow_text(
        &msg,
        &rusty_termcolor::effects::EffectSettings::default(),
    );
    print!("{}", colors::RESET);
    Ok(vm)
}

pub fn stdlib_spinner_text(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::WHITE)
}

pub fn stdlib_spinner_text_red(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::RED)
}

pub fn stdlib_spinner_text_green(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::GREEN)
}

pub fn stdlib_spinner_text_blue(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::BLUE)
}

pub fn stdlib_spinner_text_magenta(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::MAGENTA)
}

pub fn stdlib_spinner_text_cyan(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::CYAN)
}

pub fn stdlib_spinner_text_yellow(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, colors::YELLOW)
}

pub fn stdlib_spinner_text_random(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_spinner_text_base(vm, rusty_termcolor::colors::random_pleasing_color())
}

pub fn stdlib_console_text(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::WHITE)
}

pub fn stdlib_console_text_red(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::RED)
}

pub fn stdlib_console_text_green(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::GREEN)
}

pub fn stdlib_console_text_blue(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::BLUE)
}

pub fn stdlib_console_text_magenta(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::MAGENTA)
}

pub fn stdlib_console_text_cyan(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::CYAN)
}

pub fn stdlib_console_text_yellow(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, colors::YELLOW)
}

pub fn stdlib_console_text_random(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_base(vm, rusty_termcolor::colors::random_pleasing_color())
}

pub fn stdlib_console_text_ok(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_fade_base(vm, colors::WHITE, colors::GREEN)
}

pub fn stdlib_console_text_warn(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_fade_base(vm, colors::GREEN, colors::YELLOW)
}

pub fn stdlib_console_text_error(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_fade_base(vm, colors::YELLOW, colors::RED)
}

pub fn stdlib_console_text_matrix_green(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_console_text_matrix_base(vm, colors::GREEN)
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
    let _ = bc.vm.register_inline("spinner.text.red".to_string(), stdlib_spinner_text_red);
    let _ = bc.vm.register_inline("spinner.text.green".to_string(), stdlib_spinner_text_green);
    let _ = bc.vm.register_inline("spinner.text.blue".to_string(), stdlib_spinner_text_blue);
    let _ = bc.vm.register_inline("spinner.text.magenta".to_string(), stdlib_spinner_text_magenta);
    let _ = bc.vm.register_inline("spinner.text.cyan".to_string(), stdlib_spinner_text_cyan);
    let _ = bc.vm.register_inline("spinner.text.yellow".to_string(), stdlib_spinner_text_yellow);
    let _ = bc.vm.register_inline("spinner.text.randomcolor".to_string(), stdlib_spinner_text_random);

    let _ = bc.vm.register_inline("console.text".to_string(), stdlib_console_text);
    let _ = bc.vm.register_inline("console.text.red".to_string(), stdlib_console_text_red);
    let _ = bc.vm.register_inline("console.text.green".to_string(), stdlib_console_text_green);
    let _ = bc.vm.register_inline("console.text.blue".to_string(), stdlib_console_text_blue);
    let _ = bc.vm.register_inline("console.text.magenta".to_string(), stdlib_console_text_magenta);
    let _ = bc.vm.register_inline("console.text.cyan".to_string(), stdlib_console_text_cyan);
    let _ = bc.vm.register_inline("console.text.yellow".to_string(), stdlib_console_text_yellow);
    let _ = bc.vm.register_inline("console.text.randomcolor".to_string(), stdlib_console_text_random);
    let _ = bc.vm.register_inline("console.text.ok".to_string(), stdlib_console_text_ok);
    let _ = bc.vm.register_inline("console.text.warning".to_string(), stdlib_console_text_warn);
    let _ = bc.vm.register_inline("console.text.error".to_string(), stdlib_console_text_error);

    let _ = bc.vm.register_inline("console.text.matrix".to_string(), stdlib_console_text_matrix_green);
    let _ = bc.vm.register_inline("console.text.rainbow".to_string(), stdlib_console_text_rainbow);
    drop(bc);
}
