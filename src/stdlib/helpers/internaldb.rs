extern crate log;
use crate::stdlib::functions::internaldb;

pub fn internaldb_version() -> String {
    let db = match internaldb::DB.lock() {
        Ok(db) => db,
        Err(_) => {
            return "unknown".to_string();
        }
    };
    let res = match db.version() {
        Ok(version) => version,
        Err(_) => "unknown".to_string(),
    };
    drop(db);
    res
}
