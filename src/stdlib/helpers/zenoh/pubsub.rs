extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use zenoh;
use zenoh::Wait;
use easy_error::{Error, bail, ensure};

pub fn zenoh_pub(session: zenoh::Session, key: String, value: Value) -> Result<zenoh::Session, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenog config making: {}", err),
    };
    let is_distributed = cli.distributed;
    drop(cli);
    ensure!(is_distributed, "BUND must be in distributed mode. You dhall pass --distributed to CLI");
    let publisher = match session.declare_publisher(&key).wait() {
        Ok(publisher) => publisher,
        Err(err) => bail!("BUS pub error creating publisher: {}", err),
    };
    let data = match value.to_binary() {
        Ok(data) => data,
        Err(err) => bail!("BUS pub error to casting binary data: {}", err),
    };
    match publisher
            .put(data)
            .encoding(zenoh::bytes::Encoding::ZENOH_BYTES)
            .wait() {
        Ok(_) => {},
        Err(err) => bail!("BUS pub publishing error: {}", err),
    }
    Ok(session)
}
