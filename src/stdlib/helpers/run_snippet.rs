use bundcore::bundcore::Bund;
use crate::cmd;
use crate::stdlib::helpers;


pub fn run_snippet_for_cmd(bc: &mut Bund, snippet: String, cli: &cmd::Cli) {
    let code = format!("{}", &snippet);
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
            helpers::print_error::print_error(err, cli);
        }
    }
}

pub fn run_snippet_for_script(bc: &mut Bund, snippet: String, cli: &cmd::Cli) {
    let code = format!("{}", &snippet);
    match bc.eval(code) {
        Ok(_) => {}
        Err(err) => {
            helpers::print_error::print_error(err, cli);
        }
    }
}
