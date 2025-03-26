extern crate log;
use crate::cmd;
use zenoh;
use zenoh::config::{Config};
use easy_error::{Error, bail, ensure};

pub fn zenoh_config() -> Result<Config, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenog config making: {}", err),
    };
    let is_distributed = cli.distributed;
    ensure!(is_distributed, "BUND must be in distributed mode. You dhall pass --distributed to CLI");
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
