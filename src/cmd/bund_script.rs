extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;
use crate::stdlib::helpers;

pub fn run(bc: &mut Bund, cli: &cmd::Cli, script_arg: &cmd::Script) {
    log::debug!("SCRIPT::run() reached");

    if script_arg.source.stdin {
        helpers::run_snippet::run_snippet_for_cmd(bc, helpers::file_helper::get_file_from_stdin());
    } else {
        match &script_arg.source.eval {
            Some(snippet) => {
                helpers::run_snippet::run_snippet_for_cmd(bc, snippet.to_string());
            }
            None => {
                println!("No input provided");
            }
        }
    }
}
