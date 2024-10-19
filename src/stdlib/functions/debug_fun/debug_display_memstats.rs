extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use memory_stats::memory_stats;
use humansize::{format_size, DECIMAL};

pub fn debug_display_memstat_color() {
    if let Some(usage) = memory_stats() {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .add_row(vec![
                Cell::new("Current physical memory usage").fg(Color::Blue), Cell::new(format_size(usage.physical_mem, DECIMAL)).fg(Color::White),
            ])
            .add_row(vec![
                Cell::new("Current virtual memory usage").fg(Color::Blue), Cell::new(format_size(usage.virtual_mem, DECIMAL)).fg(Color::White),
            ]);
        println!("{table}");
    } else {
        println!("Couldn't get the current memory usage.");
    }
}

pub fn debug_display_memstat_nocolor() {
    if let Some(usage) = memory_stats() {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .add_row(vec![
                Cell::new("Current physical memory usage"), Cell::new(format_size(usage.physical_mem, DECIMAL)),
            ])
            .add_row(vec![
                Cell::new("Current virtual memory usage"), Cell::new(format_size(usage.virtual_mem, DECIMAL)),
            ]);
        println!("{table}");
    } else {
        println!("Couldn't get the current memory usage.");
    }
}

pub fn stdlib_debug_display_memstat_color(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_memstat_color();
    Ok(vm)
}

pub fn stdlib_debug_display_memstat_nocolor(vm: &mut VM) -> Result<&mut VM, Error> {
    debug_display_memstat_nocolor();
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
        let _ = bc.vm.register_inline("debug.display_memstat".to_string(), stdlib_debug_display_memstat_nocolor);
    } else {
        let _ = bc.vm.register_inline("debug.display_memstat".to_string(), stdlib_debug_display_memstat_color);
    }
    drop(bc);
}
