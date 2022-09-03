extern crate log;
use crate::cmd;
use crate::lang;

pub fn run_run(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("run_run() reached");
    for code in a {
        run_line(&c, &code);
    }
}

fn run_line(_c: &cmd::Cli, line: &String) {
    lang::parse(&line)
}
