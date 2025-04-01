extern crate log;
use crate::cmd;
use zenoh;
use zenoh::config::{Config};
use easy_error::{Error, bail, ensure};

pub fn zenoh_config() -> Result<Config, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenoh config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You shall pass --distributed to CLI");
    let config = match Config::from_file(cli.bus.bus_config.clone()) {
        Ok(config) => config,
        Err(err) => {
            drop(cli);
            bail!("Error configuring BUS: {}", err);
        }
    };
    drop(cli);
    Ok(config)
}

pub fn get_receiving_path(suffix: String) -> Result<String, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenoh config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You shall pass --distributed to CLI");
    let res = format!("{}/{}/{}", &cli.bus.receiving, &cli.bus.nodeid, &suffix);
    drop(cli);
    Ok(res)
}

pub fn get_globals_path(name: String) -> Result<String, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenoh config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You shall pass --distributed to CLI");
    let res = format!("{}/{}", &cli.bus.globals_prefix, &name);
    drop(cli);
    Ok(res)
}

pub fn get_outcome_path(execution_id: String) -> Result<String, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenoh config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You shall pass --distributed to CLI");
    let res = format!("{}/{}/{}", &cli.bus.outcome_prefix, &cli.bus.nodeid, &execution_id);
    drop(cli);
    Ok(res)
}

pub fn get_scripts_path(script_name: String) -> Result<String, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenoh config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You shall pass --distributed to CLI");
    let res = format!("{}/{}", &cli.bus.scripts_prefix, &script_name);
    drop(cli);
    Ok(res)
}
