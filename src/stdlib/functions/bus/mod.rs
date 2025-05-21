extern crate log;
use crate::cmd;
use rust_dynamic::value::Value;
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;
use zenoh;
use crate::stdlib::helpers;
use crossbeam::channel::{Sender, Receiver, unbounded};
use easy_error::{Error, bail};

pub mod crossbus;
pub mod globals;

lazy_static! {
    static ref PIPES: Mutex<BTreeMap<String,(Sender<Vec<u8>>, Receiver<Vec<u8>>)>> = {
        let e: Mutex<BTreeMap<String,(Sender<Vec<u8>>, Receiver<Vec<u8>>)>> = Mutex::new(BTreeMap::new());
        e
    };
}

lazy_static! {
    pub static ref ZENOH: Mutex<zenoh::Session> = {
        log::debug!("Initializing default BUS connection");
        let config = match helpers::zenoh::conf::zenoh_config() {
            Ok(config) => config,
            Err(err) => {
                log::error!("ERROR establishing BUS config: {}", err);
                std::process::exit(10);
            }
        };
        let session = match helpers::zenoh::session::zenoh_session(config) {
            Ok(session) => session,
            Err(err) => {
                log::error!("ERROR connecting to the BUS: {}", err);
                std::process::exit(10);
            }
        };
        Mutex::new(session)
    };
}

fn pipes_init() {
    log::debug!("Create default bus pipes");
    let mut q = PIPES.lock().unwrap();
    q.insert("in".to_string(), unbounded::<Vec<u8>>());
    q.insert("out".to_string(), unbounded::<Vec<u8>>());
    q.insert("force_exit".to_string(), unbounded::<Vec<u8>>());
    q.insert("tts_in".to_string(), unbounded::<Vec<u8>>());
    drop(q);
}

pub fn create_bus(n: String) {
    log::debug!("Create bus pipe: {}", &n);
    let mut q = PIPES.lock().unwrap();
    q.insert(n.to_string(), unbounded::<Vec<u8>>());
    drop(q);
}

pub fn ensure_bus(n: String) -> bool {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&n) {
        q.insert(n.to_string(), unbounded::<Vec<u8>>());
    }
    let (_, r) = q.get_mut(&n).unwrap();
    if r.is_empty() {
        drop(q);
        return false;
    }
    drop(q);
    return true;
}

pub fn bus_is_empty(k: String) -> Result<bool, Error> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        bail!("bus no pipe: {}", &k);
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        drop(q);
        return Result::Ok(true);
    }
    drop(q);
    Result::Ok(false)
}

pub fn bus_push(k: String, d: Value) -> Result<bool, Error> {
    match d.to_binary() {
        Ok(res) => {
            let mut q = PIPES.lock().unwrap();
            if ! q.contains_key(&k) {
                log::trace!("new bus::internal::pipe : {}", &k);
                let (s,r) = unbounded::<Vec<u8>>();
                match s.send(res) {
                    Ok(_) => q.insert(k, (s,r)),
                    Err(err) => {
                        drop(q);
                        bail!("bus::internal::pipe error: {}", err);
                    }
                };
            } else {
                let (s,_) = q.get_mut(&k).unwrap();
                match s.send(res) {
                    Ok(_) => {},
                    Err(err) => {
                        drop(q);
                        bail!("bus::internal::pipe error: {}", err);
                    }
                }
            }
            drop(q);
        }
        Err(err) => {
            let msg = format!("Error enveloping data: {}", err);
            log::error!("{}", &msg);
            bail!("{}", &msg);
        }
    }
    Result::Ok(true)
}

pub fn bus_pull(k: String) -> Result<Value, Error> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        bail!("bus::internal::pipe no pipe: {}", &k);
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        return Ok(Value::nodata());
    }
    match r.recv() {
        Ok(res) => {
            match Value::from_binary(res) {
                Ok(val) => {
                    drop(q);
                    return Result::Ok(val);
                }
                Err(err) => {
                    let msg = format!("Error converting from binary: {}", err);
                    log::error!("{}", &msg);
                    bail!("{}", &msg);
                }
            }
        }
        Err(err) => bail!("bus::internal::pipe {} can not recv: {}", &k, &err),
    }
}

pub fn init_stdlib(cli: &cmd::Cli) {
    pipes_init();
    if cli.distributed {
        let z = ZENOH.lock().unwrap();
        drop(z);
    }
    crossbus::init_stdlib(cli);
    globals::init_stdlib(cli);
}
