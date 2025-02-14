extern crate log;
use std::collections::HashMap;
use duckdb::types::ValueRef;
use crate::stdlib::functions::internaldb;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_internaldb_sql(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline SQL");
    }
    let query_val = match vm.stack.pull() {
        Some(query_val) => query_val,
        None => bail!("SQL: NO DATA #1"),
    };
    let query_str = match query_val.cast_string() {
        Ok(query_str) => query_str,
        Err(err) => bail!("SQL: casting query returns error: {}", err),
    };
    let db = match internaldb::DB.lock() {
        Ok(db) => db,
        Err(err) => bail!("SQL: getting reference to internal database returns error: {}", err),
    };
    log::debug!("SQL: preparing query for internaldb.sql: {}", &query_str);
    let mut stmt = match db.prepare(&query_str) {
        Ok(stmt) => stmt,
        Err(err) => {
            drop(db);
            bail!("SQL: prepare statement returns error: {}", err)
        },
    };

    let mut res = Value::list();
    match stmt.query([]) {
        Ok(mut rows) => {
            let column_count = match rows.as_ref() {
                Some(r) => r.column_count(),
                None => bail!("SQL: can not get column count"),
            };
            let mut columns_map: HashMap<usize, String> = HashMap::new();
            for c in 0..column_count {
                let name = match rows.as_ref() {
                    Some(n) => n.column_name(c).expect("valid column index"),
                    None => bail!("SQL: can not get column name"),
                };
                columns_map.insert(c, name.to_string());
            }
            loop {
                match rows.next() {
                    Ok(Some(row)) => {
                        let mut bund_row = Value::dict();
                        for c in 0..column_count {
                            let value = row.get_ref(c);
                            let name = columns_map.get(&c).unwrap();
                            match value {
                                Ok(value) => {
                                    match value {
                                        ValueRef::Null => {
                                            bund_row = bund_row.set(name, Value::nodata());
                                        }
                                        ValueRef::BigInt(i) => {
                                            bund_row = bund_row.set(name, Value::from_int(i));
                                        }
                                        ValueRef::Int(i) => {
                                            bund_row = bund_row.set(name, Value::from_int(i as i64));
                                        }
                                        ValueRef::Double(f) => {
                                            bund_row = bund_row.set(name, Value::from_float(f));
                                        }
                                        ValueRef::Float(f) => {
                                            bund_row = bund_row.set(name, Value::from_float(f as f64));
                                        }
                                        ValueRef::Boolean(b) => {
                                            bund_row = bund_row.set(name, Value::from_bool(b));
                                        }
                                        ValueRef::Text(s) => {
                                            let string_data = match std::str::from_utf8(s) {
                                                Ok(string_data) => string_data,
                                                Err(err) => bail!("SQL error converting string data: {}", err),
                                            };
                                            bund_row = bund_row.set(name, Value::from_string(string_data));
                                        }
                                        ValueRef::Blob(b) => {
                                            bund_row = bund_row.set(name, Value::from_binary(b.to_vec())?);
                                        }
                                        _ => {
                                            log::warn!("SQL: unprocessed datatype in result: {:?}", &value);
                                            continue;
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("SQL error get field: {}", err);
                                }
                            }
                        }
                        res = res.push(bund_row);
                    }
                    Ok(None) => break,
                    Err(err) => {
                        bail!("SQL error getting row: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            bail!("SQL: Error execution of SQL query: {}", err);
        }
    }
    vm.stack.push(res);
    drop(db);
    Ok(vm)
}
