extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;
use crate::stdlib::helpers;


pub fn run(bc: &mut Bund, cli: &cmd::Cli, eval_arg: &cmd::Eval) {
    log::debug!("EVAL::run() reached");

    if eval_arg.source.stdin {
        helpers::run_snippet::run_snippet_for_cmd(bc, helpers::file_helper::get_file_from_stdin(), cli);
    } else {
        match &eval_arg.source.eval {
            Some(snippet) => {
                helpers::run_snippet::run_snippet_for_cmd(bc, snippet.to_string(), cli);
            }
            None => {
                println!("No input provided");
            }
        }
    }
}
