extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use crate::stdlib::helpers;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn bund_cluster_schedule(_cli: &cmd::Cli, _bund_cluster_arg: &cmd::Cluster) {

}

fn bund_cluster_actor(_cli: &cmd::Cli, _bund_cluster_arg: &cmd::Cluster) {

}

fn bund_cluster_publish(_cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    let name = match &bund_cluster_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };
    let snippet = match helpers::file_helper::get_snippet(
        bund_cluster_arg.source.stdin,
        bund_cluster_arg.source.eval.clone(),
        bund_cluster_arg.source.file.clone(),
        bund_cluster_arg.source.url.clone(),
    ) {
        Some(snippet) => snippet,
        None => {
            log::error!("No script has been provided or found for publishing");
            return;
        }
    };
    let config = match helpers::zenoh::conf::zenoh_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("{}", err);
            return;
        }
    };
    let session = match helpers::zenoh::session::zenoh_session(config) {
        Ok(session) => session,
        Err(err) => {
            log::error!("GLOBAL returs: {}", err);
            return;
        }
    };
    let key = match helpers::zenoh::conf::get_globals_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("GLOBAL: error setting KEY: {}", err);
            return;
        }
    };
    let receiving = match helpers::zenoh::conf::get_scripts_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("PUBLISH: error setting KEY: {}", err);
            return;
        }
    };
    let from_addr = Value::from_string(receiving.clone());
    let to_addr = Value::from_string(&key);
    let payload = Value::message(from_addr, to_addr, Value::from_string(snippet));
    match helpers::zenoh::putget::zenoh_put(session, key.clone(), payload) {
        Ok(_) => {},
        Err(err) => {
            log::error!("--publish returns: {}", err);
            return;
        }
    };
    log::debug!("SCRIPT is set for key {}", &key);
}

fn bund_cluster_download(_cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    let name = match &bund_cluster_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };
    let config = match helpers::zenoh::conf::zenoh_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("{}", err);
            return;
        }
    };
    let session = match helpers::zenoh::session::zenoh_session(config) {
        Ok(session) => session,
        Err(err) => {
            log::error!("GLOBAL returs: {}", err);
            return;
        }
    };
    let key = match helpers::zenoh::conf::get_globals_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("GLOBAL: error setting KEY: {}", err);
            return;
        }
    };
    match helpers::zenoh::putget::zenoh_get(session, key.to_string()) {
        Ok(value) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic);
                for n in value {
                    let val = match n.at(2) {
                        Some(val) => val,
                        None => continue,
                    };
                    for v in val {
                        table
                        .add_row(vec![
                            Cell::new(v).fg(Color::White),
                        ]);
                    }
                }
            println!("{table}");
        },
        Err(err) => {
            log::error!("Error in GET data from {}: {}", &key, err);
            return
        }
    }
}

#[time_graph::instrument]
pub fn run(cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    log::debug!("BUND_CLUSTER::run() reached");

    if bund_cluster_arg.command.schedule {
        bund_cluster_schedule(&cli, &bund_cluster_arg);
    } else if bund_cluster_arg.command.actor {
        bund_cluster_actor(&cli, &bund_cluster_arg);
    } else if bund_cluster_arg.command.publish {
        bund_cluster_publish(&cli, &bund_cluster_arg);
    } else if bund_cluster_arg.command.download {
        bund_cluster_download(&cli, &bund_cluster_arg);
    } else {
        log::error!("Unknown CLUSTER command");
    }
}
