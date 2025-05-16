extern crate log;
use crate::cmd;
use rust_dynamic::types::*;
use std::path::Path;
use scan_dir::ScanDir;
use execution_time::ExecutionTime;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use crate::stdlib::helpers;

#[time_graph::instrument]
fn run_test_file(file: String) -> bool {
    if file.ends_with(".bund") {
        match helpers::file_helper::get_file_from_relative_file(file.clone()) {
            Some(script) => {
                return run_test_case(script, file.clone());
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

#[time_graph::instrument]
fn run_test_case(script: String, source: String) -> bool {
    let timer = ExecutionTime::start();
    match helpers::run_snippet::run_snippet_and_return_value(script) {
        Ok((value, is_desc, stack_len)) => {
            let elapsed_time = timer.get_elapsed_time();
            let res = match value.cast_bool() {
                Ok(res) => res,
                Err(err) => {
                    log::error!("Error casting BOOL value: {}", err);
                    return false;
                }
            };
            let mut table = Table::new();
            let desc = match is_desc {
                Some(is_desc) => match is_desc.conv(STRING) {
                    Ok(desc) => match desc.cast_string() {
                        Ok(desc) => desc,
                        Err(err) => format!("Error getting description: {}", err),
                    },
                    Err(err) => format!("Error casting description: {}", err),
                },
                None => "NO DESCRIPTION".to_string(),
            };
            let test_status = if res {
                if stack_len > 0 {
                    Cell::new(res).fg(Color::Yellow)
                } else {
                    Cell::new(res).fg(Color::Green)
                }
            } else {
                Cell::new(res).fg(Color::Red)
            };
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_row(vec![
                    Cell::new("Test case status").fg(Color::White), test_status,
                ])
                .add_row(vec![
                    Cell::new("Test case source").fg(Color::Blue), Cell::new(&source).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("Elapsed time").fg(Color::Blue), Cell::new(&elapsed_time).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("Test case description").fg(Color::Blue), Cell::new(&desc).fg(Color::White),
                ]);
            println!("{table}");
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
                    if run_test_case(script, c.clone()) {
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
