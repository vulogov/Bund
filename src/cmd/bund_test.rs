extern crate log;
use crate::cmd;
use std::path::Path;
use scan_dir::ScanDir;
use crate::stdlib::helpers;

fn run_test_file(file: String) -> bool {
    if file.ends_with(".bund") {
        match helpers::file_helper::get_file_from_relative_file(file.clone()) {
            Some(script) => {
                return run_test_case(script);
            }
            None => {
                log::error!("Case {} is not file", &file);
                return false;
            }
        }
    } else {
        log::warn!("File {} is not matching for the test", &file);
    }
    true
}

fn run_test_case(script: String) -> bool {
    match helpers::run_snippet::run_snippet_and_return_value(script) {
        Ok(value) => {
            let res = match value.cast_bool() {
                Ok(res) => res,
                Err(err) => {
                    log::error!("Error casting BOOL value: {}", err);
                    return false;
                }
            };
            return res;
        }
        Err(err) => {
            log::error!("{}", err);
            return false;
        }
    }
}

#[time_graph::instrument]
pub fn run(_cli: &cmd::Cli, test_arg: &cmd::Test) {
    log::debug!("TEST::run() reached");
    for c in &test_arg.cases {
        log::info!("Processing test case: [{}]", &c);
        if Path::new(c).is_file() {
            if run_test_file(c.clone()) {
                log::info!("Test case: {} OK", &c);
            } else {
                log::info!("Test case: {} FAIL", &c);
                break;
            }

        } else if Path::new(c).is_dir() {
            let _ = ScanDir::files().skip_hidden(true).skip_backup(true).walk(c, |iter| {
                for (entry, _) in iter {
                    if run_test_file(format!("{}", entry.path().display())) {
                        log::info!("Test case: {} OK", &entry.path().display());
                    } else {
                        log::info!("Test case: {} FAIL", &entry.path().display());
                        break;
                    }
                }
            });
        } else {
            match helpers::file_helper::get_file_from_uri(c.clone()) {
                Some(script) => {
                    if run_test_case(script) {
                        log::info!("Test case: {} OK", &c);
                    } else {
                        log::info!("Test case: {} FAIL", &c);
                        break;
                    }
                }
                None => {
                    log::error!("Case {} is not file, or directory or resource", &c);
                    break;
                }
            }
        }
    }
}
