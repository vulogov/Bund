extern crate log;
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use crate::stdlib::banner;
use crate::cmd;
use crate::lang;

pub fn run_shell(c: &cmd::Cli, a: &Vec<String>)  {
    log::trace!("run_shell() reached");
    println!("{}", banner::bund_banner());
    for code in a {
        shell_line(&c, &code);
    }
    let mut line = Editor::<()>::new().unwrap();
    loop {
        let readline = line.readline("[BUND > ");
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
    println!("{}", banner::banner(&"Zay Gezunt".to_string()));
}

fn shell_line(_c: &cmd::Cli, line: &String) {
    lang::parse(&line)
}
