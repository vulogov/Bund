extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use yansi::Paint;
use crate::cmd;
use crate::cmd::bund_display_banner;
use crate::stdlib::helpers;

pub fn run(cli: &cmd::Cli, _shell_arg: &cmd::Shell) {
    log::debug!("SHELL::run() reached");

    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    #[cfg(feature = "with-file-history")]
    if rl.load_history("bund_history.txt").is_err() {
        log::warn!("No previous shell history.");
    }
    println!("{}", bund_display_banner::bund_banner());
    let prompt = match cli.nocolor {
        true => format!("[BUND> "),
        false => format!("{}{}{}{}{} {} ", Paint::yellow("["), Paint::red("B"), Paint::blue("U").bold(), Paint::white("N"), Paint::cyan("D"), Paint::green(">").bold()),
    };
    loop {
        let line = rl.readline(&prompt);
        match line {
            Ok(snippet) => {
                let _ = rl.add_history_entry(snippet.as_str());
                helpers::run_snippet::run_snippet_for_cmd(snippet.to_string(), cli);
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
    #[cfg(feature = "with-file-history")]
    rl.save_history("bund_history.txt");
}
