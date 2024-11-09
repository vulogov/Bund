extern crate log;
use crate::cmd;
use crate::stdlib::helpers;
use rusqlite::{Connection};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn run_list(conn: &mut Connection, _wscript_arg: &cmd::Wscript) {
    log::debug!("WSCRIPT::run_list() reached");
    let scripts = match helpers::world::bootstrap::read_all_bootstrap_names(conn) {
        Ok(scripts) => scripts,
        Err(err) => {
            log::error!("BOOTSTRAP discovery scripts returns: {}", err);
            return;
        }
    };
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    for n in &scripts {
        table
        .add_row(vec![
            format!("{}", &n),
        ]);
    }
    println!("{table}");
}

fn run_add(conn: &mut Connection, wscript_arg: &cmd::Wscript) {
    log::debug!("WSCRIPT::run_add() reached");

    let name = match &wscript_arg.name {
        Some(name) => name,
        None => {
            log::error!("You did not specified the script name");
            return;
        }
    };
    let snippet_name = match &wscript_arg.script {
        Some(snippet_name) => snippet_name,
        None => {
            log::error!("You did not specified the script location");
            return;
        }
    };
    let snippet = match helpers::file_helper::get_file_from_uri(snippet_name.to_string()) {
        Some(snippet) => snippet,
        None => {
            log::error!("Error getting snippet from {}", &snippet_name);
            return;
        }
    };
    match helpers::world::bootstrap::store_bootstrap_script(conn, name.clone(), snippet.clone()) {
        Ok(_) => {},
        Err(err) => {
            helpers::print_error::print_error_from_str_plain(format!("{}", err));
        }
    }
}

fn run_remove(conn: &mut Connection, wscript_arg: &cmd::Wscript) {
    log::debug!("WSCRIPT::run_remove() reached");

    let name = match &wscript_arg.name {
        Some(name) => name,
        None => {
            log::error!("You did not specified the script name");
            return;
        }
    };

    match helpers::world::bootstrap::delete_bootstrap_script(conn, name.clone()) {
        Ok(_) => {},
        Err(err) => {
            helpers::print_error::print_error_from_str_plain(format!("{}", err));
        }
    }
}

fn run_export(conn: &mut Connection, wscript_arg: &cmd::Wscript) {
    log::debug!("WSCRIPT::run_export() reached");
    let name = match &wscript_arg.name {
        Some(name) => name,
        None => {
            log::error!("You did not specified the script name");
            return;
        }
    };
    match helpers::world::bootstrap::get_bootstrap_script(conn, name.clone()) {
        Ok(script) => {
            println!("{}", &script);
        }
        Err(err) => {
            log::error!("WSCRIPT export returned: {}", err);
        }
    }
}

pub fn run(cli: &cmd::Cli, wscript_arg: &cmd::Wscript) {
    log::debug!("WSCRIPT::run() reached");

    let mut conn = match helpers::world::open(wscript_arg.world.clone()) {
        Ok(conn) => conn,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    if wscript_arg.command.list {
        run_list(&mut conn, wscript_arg)
    } else if wscript_arg.command.add {
        run_add(&mut conn, wscript_arg)
    } else if wscript_arg.command.remove {
        run_remove(&mut conn, wscript_arg)
    } else if wscript_arg.command.export {
        run_export(&mut conn, wscript_arg)
    } else {
        log::error!("Unknown or unrecognized command");
    }

    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }

}
