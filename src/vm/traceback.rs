use std::time::{SystemTime, UNIX_EPOCH};

pub struct Traceback {
    ts:         std::time::SystemTime,
    r:          String,
}

impl Traceback {
    pub fn new(r: String) -> Self {
        Self {
            ts:             SystemTime::now(),
            r:              r,
        }
    }
}

impl Traceback {
    pub fn millisecond(&self) -> i64 {
        self.ts.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
    }
    pub fn elapsed(&self) -> i64 {
        self.ts.elapsed().unwrap().as_secs() as i64
    }
    pub fn rule(&self) -> &String {
        &self.r
    }
}
