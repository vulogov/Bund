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
