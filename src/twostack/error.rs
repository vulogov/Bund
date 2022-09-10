use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct BundError {
    ts:         std::time::SystemTime,
    eclass:     String,
    msg:        String,
}

impl BundError {
    pub fn new(t: String, msg: String) -> Self {
        Self {
            ts:             SystemTime::now(),
            eclass:         t,
            msg:            msg,
        }
    }
}

impl BundError {
    pub fn millisecond(&self) -> i64 {
        self.ts.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
    }
    pub fn message(&self) -> &String {
        &self.msg
    }
    pub fn class(&self) -> &String {
        &self.eclass
    }
}
