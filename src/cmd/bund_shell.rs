extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use yansi::Paint;
use crate::cmd;
use rust_dynamic::value::Value;
use crate::cmd::*;
use crate::stdlib::helpers;
use crate::stdlib::functions::{debug_fun, bund};

pub fn run(cli: &cmd::Cli, shell_arg: &cmd::Shell) {
    log::debug!("SHELL::run() reached");

    match bund::bund_args::ARGS.lock() {
        Ok(mut args) => {
            for a in &shell_arg.args {
                let _ = args.push_inplace(Value::from_string(a.clone()));
            }
            drop(args);
        }
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    }
    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    // #[cfg(feature = "with-file-history")]
    if rl.load_history("bund_history.txt").is_err() {
        log::warn!("No previous shell history.");
    }
    println!("{}", bund_display_banner::bund_banner());
    if cli.nocolor {
        debug_fun::debug_display_hostinfo::debug_display_hostinfo_nocolor();
        if cli.distributed {
            debug_fun::debug_display_distributed_info::debug_display_distributed_info_nocolor();
        } else {
            log::info!("SHELL is running in LOCAL mode");
        }
    } else {
        debug_fun::debug_display_hostinfo::debug_display_hostinfo_color();
        if cli.distributed {
            debug_fun::debug_display_distributed_info::debug_display_distributed_info_color();
        } else {
            log::info!("SHELL is running in LOCAL mode");
        }
    }
    let prompt = match cli.nocolor {
        true => format!("[BUND> "),
        false => format!("{}{}{}{}{} {} ", Paint::yellow("["), Paint::red("B"), Paint::blue("U").bold(), Paint::white("N"), Paint::cyan("D"), Paint::green(">").bold()),
    };
    loop {
        let line = rl.readline(&prompt);
        match line {
            Ok(snippet) => {
                let _ = rl.add_history_entry(snippet.as_str());
                if shell_arg.as_script {
                    helpers::run_snippet::run_snippet_for_script(snippet.to_string(), cli);
                } else {
                    helpers::run_snippet::run_snippet_for_cmd(snippet.to_string(), cli);
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
                helpers::print_error::print_error_from_str(format!("{}", err), cli);
                break
            }
        }
    }
    log::debug!("Saving shell history...");
    // #[cfg(feature = "with-file-history")]
    let _ = rl.save_history("bund_history.txt");
    println!("{}", bund_display_banner::banner_small(&"Ir vet hobn a gut tog".to_string()));
}
