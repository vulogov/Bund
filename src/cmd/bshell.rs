extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use crate::cmd;

pub fn run_shell(c: &cmd::Cli, _a: &Vec<String>)  {
    log::trace!("run_shell() reached");
    let mut line = Editor::<()>::new().unwrap();
    loop {
        let readline = line.readline("[BUND> ");
        match readline {
            Ok(line) => {
                shell_line(&c, &line)
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
                log::error!("Error: {:?}", err);
                break
            }
        }
    }
}

fn shell_line(_c: &cmd::Cli, _line: &String) {

}
