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

pub fn debug_display_hostinfo_color() {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => {
            log::error!("Error locking BUND CLI: {}", err);
            return;
        }
    };
    let is_distributed = cli.distributed;
    drop(cli);
    let mut table = Table::new();
    let hostname = match sys_metrics::host::get_hostname() {
        Ok(hostname) => hostname,
        Err(_) => "Unknown".to_string(),
    };
    let kernel_version = match sys_metrics::host::get_kernel_version() {
        Ok(kernel_version) => kernel_version,
        Err(_) => "Unknown".to_string(),
    };
    let os_version = match sys_metrics::host::get_os_version() {
        Ok(os_version) => os_version,
        Err(_) => "Unknown".to_string(),
    };
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("rust_dynamic version").fg(Color::Green), Cell::new(&rust_dynamic::version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("rust_multistack version").fg(Color::Green), Cell::new(&rust_multistack::version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("rust_multistackvm version").fg(Color::Green), Cell::new(&rust_multistackvm::version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("bundcore version").fg(Color::Green), Cell::new(&bundcore::version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("bund_language_parser").fg(Color::Green), Cell::new(&bund_language_parser::version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("internaldb").fg(Color::Green), Cell::new(&helpers::internaldb::internaldb_version()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Distributed mode").fg(Color::Blue), Cell::new(&is_distributed).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Hostname").fg(Color::Blue), Cell::new(&hostname.clone()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("OS version").fg(Color::Blue), Cell::new(&os_version.clone()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Virtualization").fg(Color::Blue), Cell::new(&functions::sysinfo::virt::bund_sysinfo_virtualization().clone()).fg(Color::White),
        ])
        .add_row(vec![
            Cell::new("Kernel version").fg(Color::Blue), Cell::new(&kernel_version.clone()).fg(Color::White),
        ]);
    println!("{table}");
}

pub fn debug_display_hostinfo_nocolor() {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => {
            log::error!("Error locking BUND CLI: {}", err);
            return;
        }
    };
    let is_distributed = cli.distributed;
    drop(cli);
    let mut table = Table::new();
    let hostname = match sys_metrics::host::get_hostname() {
        Ok(hostname) => hostname,
        Err(_) => "Unknown".to_string(),
    };
    let kernel_version = match sys_metrics::host::get_kernel_version() {
        Ok(kernel_version) => kernel_version,
        Err(_) => "Unknown".to_string(),
    };
    let os_version = match sys_metrics::host::get_os_version() {
        Ok(os_version) => os_version,
        Err(_) => "Unknown".to_string(),
    };
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("rust_dynamic version"), Cell::new(&rust_dynamic::version()),
        ])
        .add_row(vec![
            Cell::new("rust_multistack version"), Cell::new(&rust_multistack::version()),
        ])
        .add_row(vec![
            Cell::new("rust_multistackvm version"), Cell::new(&rust_multistackvm::version()),
        ])
        .add_row(vec![
            Cell::new("bundcore version"), Cell::new(&bundcore::version()),
        ])
        .add_row(vec![
            Cell::new("bund_language_parser version"), Cell::new(&bund_language_parser::version()),
        ])
        .add_row(vec![
            Cell::new("internaldb"), Cell::new(&helpers::internaldb::internaldb_version()),
        ])
        .add_row(vec![
            Cell::new("Distributed mode"), Cell::new(&is_distributed),
        ])
        .add_row(vec![
            Cell::new("Hostname"), Cell::new(&hostname.clone()),
        ])
        .add_row(vec![
            Cell::new("OS version"), Cell::new(&os_version.clone()),
        ])
        .add_row(vec![
            Cell::new("Virtualization"), Cell::new(&functions::sysinfo::virt::bund_sysinfo_virtualization().clone()),
        ])
        .add_row(vec![
            Cell::new("Kernel version"), Cell::new(&kernel_version.clone()),
        ]);
    println!("{table}");
}

pub fn stdlib_debug_display_hostinfo_color(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_hostinfo_color();
    Ok(vm)
}

pub fn stdlib_debug_display_hostinfo_nocolor(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_hostinfo_nocolor();
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
        let _ = bc.vm.register_inline("debug.display_hostinfo".to_string(), stdlib_debug_display_hostinfo_nocolor);
    } else {
        let _ = bc.vm.register_inline("debug.display_hostinfo".to_string(), stdlib_debug_display_hostinfo_color);
    }
    drop(bc);
}
