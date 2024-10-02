extern crate log;
use crate::cmd;
use crate::stdlib::helpers;


pub fn run(cli: &cmd::Cli, eval_arg: &cmd::Eval) {
    log::debug!("EVAL::run() reached");

    if eval_arg.source.stdin {
        helpers::run_snippet::run_snippet_for_cmd(helpers::file_helper::get_file_from_stdin(), cli);
    } else {
        match &eval_arg.source.eval {
            Some(snippet) => {
                helpers::run_snippet::run_snippet_for_cmd(snippet.to_string(), cli);
            }
            None => {
                println!("No input provided");
            }
        }
    }
}
