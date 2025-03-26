extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use zenoh;
use zenoh::Wait;
use easy_error::{Error, bail, ensure};

pub fn zenoh_put(session: zenoh::Session, key: String, value: Value) -> Result<zenoh::Session, Error> {
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
    log::debug!("Prepared {} bytes for sending to {}", &data.len(), &key);
    match publisher
            .put(data)
            .encoding(zenoh::bytes::Encoding::ZENOH_BYTES)
            .wait() {
        Ok(_) => {},
        Err(err) => bail!("BUS PUT publishing error: {}", err),
    }
    Ok(session)
}

pub fn zenoh_get(session: zenoh::Session, key: String) -> Result<Value, Error> {
    let cli = match cmd::CLI.lock() {
        Ok(cli) => cli,
        Err(err) => bail!("Error locking BUND CLI for Zenog config making: {}", err),
    };
    let is_distributed = cli.distributed;
    drop(cli);
    ensure!(is_distributed, "BUND must be in distributed mode. You dhall pass --distributed to CLI");
    let replies = match session.get(&key).wait() {
        Ok(replies) => replies,
        Err(err) => bail!("BUS GET replies error: {}", err),
    };
    let mut res = Value::list();
    while let Ok(reply) = replies.recv() {
        match reply.result() {
            Ok(value_data) => {
                let message = match Value::from_binary(value_data.payload().to_bytes().into_owned()) {
                    Ok(message) => message,
                    Err(err) => bail!("BUT GET error unwrapping envelope: {}", err),
                };
                let to_addr = match message.get("to") {
                    Ok(to_addr) => to_addr,
                    Err(err) => bail!("BUT GET error gedtting destination: {}", err),
                };
                let payload = match message.get("payload") {
                    Ok(to_addr) => to_addr,
                    Err(err) => bail!("BUT GET error gedtting payload: {}", err),
                };
                let mut row = Value::list();
                row = row.push(to_addr);
                row = row.push(payload);
                res = res.push(row);
            }
            Err(err) => bail!("BUT GET receiving error: {}", err),
        }
    }
    Ok(res)
}
