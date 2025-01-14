extern crate log;
use crate::cmd;
use time_graph;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::{bund};
use crate::stdlib::functions::ai::linguistic;

#[time_graph::instrument]
pub fn run(cli: &cmd::Cli, script_arg: &cmd::Script) {
    log::debug!("SCRIPT::run() reached");

    match bund::bund_args::ARGS.lock() {
        Ok(mut args) => {
            for a in &script_arg.args {
                let _ = args.push_inplace(Value::from_string(a.clone()));
            }
            drop(args);
        }
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    }

    if script_arg.ai.ai_preload_languages {
        log::debug!("Triggering loading of languages for classifier at startup");
        linguistic::languages_preload();
    }

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
