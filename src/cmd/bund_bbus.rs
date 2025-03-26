extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use crate::stdlib::helpers;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn bund_bus_publish(_cli: &cmd::Cli, _bund_bus_arg: &cmd::Bbus, _value: Value) {
    log::debug!("BUND_BUS::bund_bus_publish() reached");
}

fn bund_bus_subscribe(_cli: &cmd::Cli, _bund_bus_arg: &cmd::Bbus) {
    log::debug!("BUND_BUS::bund_bus_subscribe() reached");
}

fn bund_bus_put(cli: &cmd::Cli, bund_bus_arg: &cmd::Bbus, value: Value) {
    log::debug!("BUND_BUS::bund_bus_put() reached");
    let config = match helpers::zenoh::conf::zenoh_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error getting BUS config: {}", err);
            return;
        }
    };
    let session = match helpers::zenoh::session::zenoh_session(config) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error getting BUS session: {}", err);
            return;
        }
    };
    let bund_key = match &bund_bus_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };
    let from_addr = Value::from_string(cli.bus.receiving.clone());
    let to_addr = Value::from_string(&bund_key);
    let payload = Value::message(from_addr, to_addr, value);

    match helpers::zenoh::putget::zenoh_put(session, bund_key.to_string(), payload) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error in PUT data to {}: {}", &bund_key, err);
            return
        }
    }
}

fn bund_bus_get(_cli: &cmd::Cli, bund_bus_arg: &cmd::Bbus) {
    log::debug!("BUND_BUS::bund_bus_get() reached");
    let config = match helpers::zenoh::conf::zenoh_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error getting BUS config: {}", err);
            return;
        }
    };
    let session = match helpers::zenoh::session::zenoh_session(config) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Error getting BUS session: {}", err);
            return;
        }
    };
    let bund_key = match &bund_bus_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };

    match helpers::zenoh::putget::zenoh_get(session, bund_key.to_string()) {
        Ok(value) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic);
                for n in value {
                    let key = match n.at(1) {
                        Some(key) => key,
                        None => continue,
                    };
                    let val = match n.at(2) {
                        Some(val) => val,
                        None => continue,
                    };
                    table
                    .add_row(vec![
                        Cell::new(key).fg(Color::Green), Cell::new(val).fg(Color::White),
                    ]);
                }
            println!("{table}");
        },
        Err(err) => {
            log::error!("Error in GET data from {}: {}", &bund_key, err);
            return
        }
    }
}

#[time_graph::instrument]
pub fn run(cli: &cmd::Cli, bund_bus_arg: &cmd::Bbus) {
    log::debug!("BUND_BUS::run() reached");
    let value = match &bund_bus_arg.value {
        Some(snippet) => match helpers::run_snippet::run_snippet_and_return_value(snippet.to_string()) {
            Ok((value, _, _)) => value,
            Err(err) => {
                log::error!("Error computing value for the bus: {}", err);
                return;
            }
        },
        None => Value::nodata(),
    };
    if bund_bus_arg.command.publish {
        bund_bus_publish(&cli, &bund_bus_arg, value);
    } else if bund_bus_arg.command.subscribe {
        bund_bus_subscribe(&cli, &bund_bus_arg);
    } else if bund_bus_arg.command.put {
        bund_bus_put(&cli, &bund_bus_arg, value);
    } else if bund_bus_arg.command.get {
        bund_bus_get(&cli, &bund_bus_arg);
    } else {
        log::error!("Unknown BUS command");
    }
}
