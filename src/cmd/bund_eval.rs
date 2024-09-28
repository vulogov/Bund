extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;
use crate::stdlib::helpers;


fn run_snippet(bc: &mut Bund, snippet: String) {
    let code = format!("{} ", &snippet);
    match bc.run(code) {
        Ok(val) => {
            match val {
                Some(returned_value) => {
                    println!("{}", &returned_value);
                }
                None => {
                    log::warn!("Snippet returned no value");
                }
            }
        }
        Err(err) => {
            helpers::print_error::print_error(err);
        }
    }
}

pub fn run(bc: &mut Bund, cli: &cmd::Cli, eval_arg: &cmd::Eval) {
    log::debug!("EVAL::run() reached");

    if eval_arg.source.stdin {
        run_snippet(bc, helpers::file_helper::get_file_from_stdin());
    } else {
        match &eval_arg.source.eval {
            Some(snippet) => {
                run_snippet(bc, snippet.to_string());
            }
            None => {
                println!("No input provided");
            }
        }
    }
}
