extern crate log;
use crate::cmd;
use zenoh;
use zenoh::Wait;
use zenoh::config::{Config};
use easy_error::{Error, bail, ensure};

pub fn zenoh_session(config: Config) -> Result<zenoh::Session, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenog config making: {}", err),
    };
    let is_distributed = cli.distributed;
    drop(cli);
    ensure!(is_distributed, "BUND must be in distributed mode. You dhall pass --distributed to CLI");
    let session = match zenoh::open(config).wait() {
        Ok(session) => session,
        Err(err) => bail!("BUS session error: {}", err),
    };
    Ok(session)
}
