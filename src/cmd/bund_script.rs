extern crate log;
use crate::cmd;
use crate::stdlib::helpers;

pub fn run(cli: &cmd::Cli, script_arg: &cmd::Script) {
    log::debug!("SCRIPT::run() reached");

    if script_arg.source.stdin {
        log::debug!("Taking snippet from STDIN");
        helpers::run_snippet::run_snippet_for_script(helpers::file_helper::get_file_from_stdin(), cli);
    } else {
        match &script_arg.source.eval {
            Some(snippet) => {
                helpers::run_snippet::run_snippet_for_script(snippet.to_string(), cli);
            }
            None => {
                match &script_arg.source.file {
                    Some(snippet_file) => {
                        let snippet = match helpers::file_helper::get_file_from_file(snippet_file.to_string()) {
                            Some(snippet) => snippet,
                            None => return,
                        };
                        helpers::run_snippet::run_snippet_for_script(snippet, cli);
                    }
                    None => {
                        match &script_arg.source.url {
                            Some(snippet) => {
                                helpers::run_snippet::run_snippet_for_script(snippet.to_string(), cli);
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
