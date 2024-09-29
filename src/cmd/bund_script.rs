extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;
use crate::stdlib::helpers;

pub fn run(bc: &mut Bund, cli: &cmd::Cli, script_arg: &cmd::Script) {
    log::debug!("SCRIPT::run() reached");

    if script_arg.source.stdin {
        helpers::run_snippet::run_snippet_for_script(bc, helpers::file_helper::get_file_from_stdin(), cli);
    } else {
        match &script_arg.source.eval {
            Some(snippet) => {
                helpers::run_snippet::run_snippet_for_script(bc, snippet.to_string(), cli);
            }
            None => {
                match &script_arg.source.file {
                    Some(snippet) => {
                        helpers::run_snippet::run_snippet_for_cmd(bc, snippet.to_string(), cli);
                    }
                    None => {
                        match &script_arg.source.url {
                            Some(snippet) => {
                                helpers::run_snippet::run_snippet_for_cmd(bc, snippet.to_string(), cli);
                            }
                            None => {
                                log::error!("No sources for script were provided");
                            }
                        }
                    }
                }
            }
        }
    }
}
