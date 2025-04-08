extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use std::collections::HashMap;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use std::time::Duration;
use beanstalkc::Beanstalkc;
use serde_json::json;
use crate::stdlib::helpers;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use zenoh::Wait;

fn actor_run(job: HashMap<String, String>) {
    let name = match job.get("name") {
        Some(name) => name,
        None => {
            log::error!("NO SCRIPT NAME IN JOB");
            return;
        }
    };
    let key = match helpers::zenoh::conf::get_scripts_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("GLOBAL: error setting KEY: {}", err);
            return;
        }
    };
    match helpers::zenoh::putget::zenoh_get_internal(key.to_string()) {
        Ok(value) => {
            log::debug!("Running actor script from {}", &key.to_string());
            for n in value {
                let val = match n.at(2) {
                    Some(val) => val,
                    None => continue,
                };
                for snippet in val {
                    let snippet_str = snippet.to_string();
                    match helpers::run_snippet::run_snippet_as_actor(&snippet_str) {
                        Ok(_) => {},
                        Err(err) => {
                            helpers::print_error::format_error_plain(err);
                        }
                    }
                }
            }
        },
        Err(err) => {
            log::error!("Error in GET data from {}: {}", &key, err);
            return
        }
    }
}

fn bund_cluster_actor(cli: &cmd::Cli, _bund_cluster_arg: &cmd::Cluster) {
    let mut conn = match Beanstalkc::new()
                         .host(&cli.bus.beanstalk_host)
                         .port(cli.bus.beanstalk_port)
                         .connection_timeout(Some(Duration::from_secs(10)))
                         .connect() {
        Ok(conn) => conn,
        Err(err) => {
            log::error!("Can not connect to beanstalk at {}:{}: {}", &cli.bus.beanstalk_host, cli.bus.beanstalk_port, err);
            return;
        }
    };
    match conn.use_tube(&cli.bus.beanstalk_tube) {
        Ok(name) => log::debug!("Using BEANSTALK tube: {}", &name),
        Err(err) => {
            log::error!("Error connecting to BEANSTALK tube {}: {}", &cli.bus.beanstalk_tube, err);
            return;
        }
    }
    match conn.watch(&cli.bus.beanstalk_tube) {
        Ok(n) => log::debug!("Watching {} BEANSTALK tubes", n),
        Err(err) => {
            log::error!("Error watching BEANSTALK tube {}: {}", &cli.bus.beanstalk_tube, err);
            return;
        }
    }

    loop {
        log::debug!("In the RECV loop");
        let mut job = match conn.reserve() {
            Ok(job) => job,
            Err(err) => {
                log::error!("BEANSTALK can not reserve job: {}", err);
                break;
            }
        };
        let body: HashMap<String, String> = match std::str::from_utf8(job.body()) {
            Ok(body) => match serde_json::from_str(body) {
                Ok(json_body) => json_body,
                Err(err) => {
                    log::error!("Error parsing job body: {}", err);
                    return;
                }
            },
            Err(err) => {
                log::error!("Error converting job body: {}", err);
                return;
            }
        };
        actor_run(body);
        match job.delete() {
            Ok(_) => {},
            Err(err) => {
                log::error!("BEANSTALK can not delete job: {}", err);
                break;
            }
        }
    }
    log::debug!("Getting out of here!");
}

fn bund_cluster_schedule(cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    if bund_cluster_arg.upload {
        bund_cluster_publish(cli, bund_cluster_arg);
    }
    let mut conn = match Beanstalkc::new()
                         .host(&cli.bus.beanstalk_host)
                         .port(cli.bus.beanstalk_port)
                         .connection_timeout(Some(Duration::from_secs(10)))
                         .connect() {
        Ok(conn) => conn,
        Err(err) => {
            log::error!("Can not connect to beanstalk at {}:{}: {}", &cli.bus.beanstalk_host, cli.bus.beanstalk_port, err);
            return;
        }
    };
    match conn.use_tube(&cli.bus.beanstalk_tube) {
        Ok(name) => log::debug!("Using BEANSTALK tube: {}", &name),
        Err(err) => {
            log::error!("Error connecting to BEANSTALK tube {}: {}", &cli.bus.beanstalk_tube, err);
            return;
        }
    }
    let name = match &bund_cluster_arg.job {
        Some(name) => name,
        None => {
            log::error!("Error getting a script name");
            return;
        }
    };
    let key = match helpers::zenoh::conf::get_scripts_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("SCHEDULE: error setting KEY: {}", err);
            return;
        }
    };
    let outcome = match helpers::zenoh::conf::get_outcome_path(bund_cluster_arg.execid.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("SCHEDULE: error setting OUTCOME: {}", err);
            return;
        }
    };
    let payload = json!({
        "script": key.clone(),
        "name": name.clone(),
        "nodeid": cli.bus.nodeid.clone(),
        "hostname": cli.bus.hostname.clone(),
        "id": bund_cluster_arg.execid.clone(),
        "return": outcome.clone(),
    });
    match conn.put_default(payload.to_string().as_bytes()) {
        Ok(j_id) => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_row(vec![
                    Cell::new("NAME").fg(Color::Green),
                    Cell::new(name.clone()).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("JOB ID").fg(Color::Green),
                    Cell::new(&j_id).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("EXEC ID").fg(Color::Green),
                    Cell::new(bund_cluster_arg.execid.clone()).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("NODE ID").fg(Color::Green),
                    Cell::new(cli.bus.nodeid.clone()).fg(Color::White),
                ])
                .add_row(vec![
                    Cell::new("HOSTNAME").fg(Color::Green),
                    Cell::new(cli.bus.hostname.clone()).fg(Color::White),
                ]);
            println!("{table}");
        }
        Err(err) => {
            log::error!("SCHEDULE: error scheduling job: {}", err);
            return;
        }
    }
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
            log::error!("PUBLISH returs: {}", err);
            return;
        }
    };
    let key = match helpers::zenoh::conf::get_scripts_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("PUBLISH: error setting KEY: {}", err);
            return;
        }
    };
    let receiving = match helpers::zenoh::conf::get_receiving_path(name.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("PUBLISH: error setting RECEIVING: {}", err);
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
    let name = match &bund_cluster_arg.job {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --job");
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
    let key = match helpers::zenoh::conf::get_scripts_path(name.clone()) {
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

fn bund_cluster_push(cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    let key = match &bund_cluster_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };
    log::debug!("PUSH to {}", &key);
    let receiving = match helpers::zenoh::conf::get_receiving_path(cli.bus.nodeid.clone()) {
        Ok(key) => key,
        Err(err) => {
            log::error!("PUBLISH: error setting RECEIVING: {}", err);
            return;
        }
    };
    let from_addr = Value::from_string(receiving.clone());
    let to_addr = Value::from_string(&key);
    for v in bund_cluster_arg.args.clone() {
        let value = match helpers::run_snippet::run_snippet_and_return_value(v.to_string()) {
            Ok((value, _, _)) => value,
            Err(err) => {
                log::error!("Error computing value for the bus: {}", err);
                return;
            }
        };
        let payload = Value::message(from_addr.clone(), to_addr.clone(), value);
        match helpers::zenoh::pubsub::zenoh_pub_internal(key.to_string(), payload) {
            Ok(_) => {},
            Err(err) => {
                log::error!("PUSH: error sending: {}", err);
                return;
            }
        }
    }
    //
    // And sending NODATA which means that is no more data for now
    //
    let payload = Value::message(from_addr.clone(), to_addr.clone(), Value::nodata());
    match helpers::zenoh::pubsub::zenoh_pub_internal(key.to_string(), payload) {
        Ok(_) => {},
        Err(err) => {
            log::error!("PUSH: error sending: {}", err);
            return;
        }
    }
}

fn bund_cluster_pull(_cli: &cmd::Cli, bund_cluster_arg: &cmd::Cluster) {
    let key = match &bund_cluster_arg.key {
        Some(key) => key,
        None => {
            log::error!("Destination is not defined with --key");
            return;
        }
    };
    let job = match &bund_cluster_arg.job {
        Some(job) => job,
        None => {
            log::error!("Actor is not defined with --job");
            return;
        }
    };
    log::debug!("PULL from {}", &key);
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
            log::error!("PULL session returs: {}", err);
            return;
        }
    };
     let subscriber = match session.declare_subscriber(key).wait() {
         Ok(subscriber) => subscriber,
         Err(err) => {
             log::error!("PULL subscriber returs: {}", err);
             return;
         }
     };
     loop {
         let mut bc = match BUND.lock() {
             Ok(bc) => bc,
             Err(err) => {
                 log::error!("Can not lock BUND: {}", err);
                 return;
             }
         };
         'outer: while let Ok(payload) = subscriber.recv() {
             let data = payload.payload().to_bytes().to_vec();
             let value = match Value::from_binary(data) {
                 Ok(value) => value,
                 Err(err) => {
                     log::error!("PULL decoding: {}", err);
                     return;
                 }
             };
             let pull_value = match value.get("payload") {
                 Ok(value) => value,
                 Err(err) => {
                     log::error!("PULL getting value: {}", err);
                     continue;
                 }
             };
             for v in pull_value {
                 if v.is_type(NODATA) {
                     break 'outer;
                 }
                 if bund_cluster_arg.stdout {
                     println!("{:?}", &v);
                 }
                 bc.vm.stack.push(v);
             }
         }
         drop(bc);
         log::debug!("Running job: {} with data from: {}", &job, &key);
         let mut body: HashMap<String, String> = HashMap::new();
         body.insert("name".to_string(), job.to_string());
         actor_run(body)
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
    } else if bund_cluster_arg.command.push {
        bund_cluster_push(&cli, &bund_cluster_arg);
    } else if bund_cluster_arg.command.pull {
        bund_cluster_pull(&cli, &bund_cluster_arg);
    } else {
        log::error!("Unknown CLUSTER command");
    }
}
