extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use bund_language_parser::bund_parse;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use yansi::Paint;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn bund_debug_print_word(word: &Value) {
    let str_value = match word.conv(STRING) {
        Ok(value) => value,
        Err(_) => Value::none(),
    };
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("Value type").fg(Color::Cyan), Cell::new(format!("{}", word.type_name())).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Value").fg(Color::Cyan), Cell::new(str_value).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Debug").fg(Color::Blue), Cell::new(format!("{:?}", word)).fg(Color::White),
        ]);
    println!("{table}");
}

pub fn bund_debugger<N: AsRef<str> + ToString>(vm: &mut VM, value: N) -> Result<&mut VM, Error> {
    let mut rl = match DefaultEditor::new() {
        Ok(line) => line,
        Err(err) => {
            helpers::print_error::print_error_from_str_plain(format!("{}", err));
            return Ok(vm);
        }
    };
    #[cfg(feature = "with-file-history")]
    if rl.load_history("bund_debug_debugger_history.txt").is_err() {
        log::warn!("No previous shell history.");
    }
    let prompt = format!("{}{} {} ", Paint::yellow("["), Paint::red("DEBUG"), Paint::white(">").bold());
    let source = format!("{}\n", value.to_string());
    match bund_parse(&source) {
        Ok(words) => {
            for word in words {
                match word.dt {
                    NONE => {
                        continue;
                    }
                    EXIT => {
                        break;
                    }
                    ERROR => {
                        match word.cast_error() {
                            Ok(error) => {
                                bail!("Detected an Error posted on the stack: {:?}", error);
                            }
                            Err(err) => {
                                bail!("Detected an Error posted on the stack, but extraction of error is failed: {}", err);
                            }
                        }
                    }
                    _ => {
                        bund_debug_print_word(&word);
                        match vm.apply(word.clone()) {
                            Ok(_) => {}
                            Err(err) => {
                                bail!("Attempt to evaluate value {:?} returned error: {}", &word, err);
                            }
                        }
                        loop {
                            let line = rl.readline(&prompt);
                            match line {
                                Ok(snippet) => {
                                    if snippet.len() == 0 {
                                        break;
                                    }
                                    let _ = rl.add_history_entry(snippet.as_str());
                                    match helpers::eval::bund_compile_and_eval(vm, snippet) {
                                        Ok(_) => continue,
                                        Err(err) => {
                                            helpers::print_error::print_error_from_str_plain(format!("{}", err));
                                        }
                                    }
                                },
                                Err(ReadlineError::Interrupted) => {
                                    log::info!("CTRL-C");
                                    break
                                },
                                Err(ReadlineError::Eof) => {
                                    log::info!("CTRL-D");
                                    break
                                },
                                Err(err) => {
                                    helpers::print_error::print_error_from_str_plain(format!("{}", err));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            bail!("{}", err);
        }
    }
    log::debug!("Saving shell history...");
    #[cfg(feature = "with-file-history")]
    rl.save_history("bund_debug_shell_history.txt");
    Ok(vm)
}

pub fn debug_debug(vm: &mut VM)  -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline DEBUG");
    }
    let debug_snippet = match vm.stack.pull() {
        Some(debug_snippet) => debug_snippet,
        None => {
            bail!("Stack is too shallow for debug()");
        }
    };
    match debug_snippet.cast_string() {
        Ok(snippet) => {
            return bund_debugger(vm, snippet);
        }
        Err(err) => {
            bail!("Casting debug snippet returns: {}", err);
        }
    }
}

pub fn stdlib_debug_debug(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_debug(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("debug".to_string(), stdlib_debug_debug);
    drop(bc);
}
