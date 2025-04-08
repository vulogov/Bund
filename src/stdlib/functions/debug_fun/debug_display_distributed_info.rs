extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use crate::stdlib::functions;
use easy_error::{Error};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use zenoh::Wait;

pub fn debug_display_distributed_info_color() {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => {
            log::error!("Error locking BUND CLI for Zenoh config making: {}", err);
            return;
        }
    };
    let is_distributed = cli.distributed;
    if ! is_distributed {
        log::error!("BUND must be in distributed mode. You shall pass --distributed to CLI");
        return;
    }
    let session = match functions::bus::ZENOH.lock() {
        Ok(session) => session,
        Err(err) => {
            log::error!("Error locking BUS connection: {}", err);
            return;
        }
    };
    let info = session.info();
    let zid = info.zid().wait();
    let rzid: Vec<_> = info.routers_zid().wait().collect();
    let pzid: Vec<_> = info.peers_zid().wait().collect();
    drop(session);
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("Node ID").fg(Color::Green), Cell::new(&cli.bus.nodeid).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Node role").fg(Color::Green), Cell::new(&cli.bus.noderole).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("ZID").fg(Color::Green), Cell::new(&zid).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Routers ZID").fg(Color::Green), Cell::new(format!("{:?}", &rzid)).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Peers ZID").fg(Color::Green), Cell::new(format!("{:?}", &pzid)).fg(Color::White),
        ])
        ;
    println!("{table}");
    drop(cli);
}

pub fn debug_display_distributed_info_nocolor() {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => {
            log::error!("Error locking BUND CLI for Zenoh config making: {}", err);
            return;
        }
    };
    let is_distributed = cli.distributed;
    if ! is_distributed {
        log::error!("BUND must be in distributed mode. You shall pass --distributed to CLI");
        return;
    }
    let session = match functions::bus::ZENOH.lock() {
        Ok(session) => session,
        Err(err) => {
            log::error!("Error locking BUS connection: {}", err);
            return;
        }
    };
    let info = session.info();
    let zid = info.zid().wait();
    let rzid: Vec<_> = info.routers_zid().wait().collect();
    let pzid: Vec<_> = info.peers_zid().wait().collect();
    drop(session);
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("Node ID"), Cell::new(&cli.bus.nodeid),
        ])
        .add_row(vec![
            Cell::new("Node role"), Cell::new(&cli.bus.noderole),
        ])
        .add_row(vec![
            Cell::new("ZID"), Cell::new(&zid),
        ])
        .add_row(vec![
            Cell::new("Routers ZID"), Cell::new(format!("{:?}", &rzid)),
        ])
        .add_row(vec![
            Cell::new("Peers ZID"), Cell::new(format!("{:?}", &pzid)),
        ])
        ;
    println!("{table}");
    drop(cli);
}

pub fn stdlib_debug_display_distributed_info_color(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_distributed_info_color();
    Ok(vm)
}

pub fn stdlib_debug_display_distributed_info_nocolor(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_distributed_info_nocolor();
    Ok(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.nocolor {
        let _ = bc.vm.register_inline("debug.display_distributed_info".to_string(), stdlib_debug_display_distributed_info_nocolor);
    } else {
        let _ = bc.vm.register_inline("debug.display_distributed_info".to_string(), stdlib_debug_display_distributed_info_color);
    }
    drop(bc);
}
