extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use prqlc;
use rusqlite::Connection;
use rusqlite::types::ValueRef;
use std::collections::HashMap;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

fn prql_to_sql(query: String) -> Result<String, Error> {
    let prql_opt = prqlc::Options{
        format: true,
        signature_comment: false,
        color: false,
        display: prqlc::DisplayOptions::Plain,
        target: prqlc::Target::Sql(Some(prqlc::sql::Dialect::SQLite)),
    };
    match prqlc::prql_to_pl(&query) {
        Ok(pl) => {
            match prqlc::pl_to_rq(pl) {
                Ok(rq) => {
                    match prqlc::rq_to_sql(rq, &prql_opt) {
                        Ok(res) => Ok(res),
                        Err(err) => bail!("PRQL.COMPILE(SQL) returns: {}", err),
                    }
                }
                Err(err) => bail!("PRQL.COMPILE(RQ) returns: {}", err),
            }
        }
        Err(err) => bail!("PRQL.COMPILE(PL) returns: {}", err),
    }

}

fn open_sqlite3(file_name: String) -> Result<Connection, Error> {
    log::debug!("CONTEXT.RUN: processing SQLITE file name: {}", &file_name);
    match Connection::open(file_name.clone()) {
        Ok(connection) => {
            return Ok(connection);
        }
        Err(err) => {
            bail!("Open world operation returns: {}", err);
        }
    }
}

pub fn stdlib_conditional_sqlite(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for sqlite");
    }
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("CONTEXT: No data file name discovered on the stack"),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT: Error name casting: {}", err),
    };
    if ! helpers::filesystem_helper::filesystem_is_file(name.clone()) {
        bail!("SQLITE: sqlite file not found: {}", &name);
    }
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("sqlite"));
    res = res.set("name", Value::from_string(name.clone()));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    let sf_val = match value.get("skip_first") {
        Ok(sf_val) => sf_val,
        Err(_) => Value::from_int(0),
    };
    let skip_first = match sf_val.cast_int() {
        Ok(skip_first) => skip_first,
        Err(err) => bail!("CONTEXT.RUN: casting SQLITE skip_first returns error: {}", err),
    };
    let name_val = match value.get("name") {
        Ok(name_val) => name_val,
        Err(err) => bail!("CONTEXT.RUN: getting SQLITE file name returns error: {}", err),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT.RUN: casting SQLITE file name returns error: {}", err),
    };

    let sql_val = match value.get("sql") {
        Ok(sql_val) => sql_val,
        Err(_) => {
            match value.get("prql") {
                Ok(prql_val) => {
                    let prql_str = match prql_val.cast_string() {
                        Ok(prql_str) => prql_str,
                        Err(err) => bail!("CONTEXT.RUN: PRQL casting returns: {}", err),
                    };
                    match prql_to_sql(prql_str) {
                        Ok(sql_val) => Value::from_string(sql_val),
                        Err(err) => bail!("{}", err),
                    }
                }
                Err(err) => bail!("CONTEXT.RUN: can not get ether SQL or PRQL query: {}", err),
            }
        },
    };
    let sql_str = match sql_val.cast_string() {
        Ok(sql_str) => sql_str,
        Err(err) => bail!("CONTEXT.RUN error casting SQL query: {}", err),
    };

    if ! helpers::filesystem_helper::filesystem_is_file(name.clone()) {
        bail!("SQLITE: sqlite file not found: {}", &name);
    }
    let lambda_val = match value.get("lambda") {
        Ok(lambda_val) => lambda_val,
        Err(_) => Value::lambda(),
    };

    let conn = match open_sqlite3(name) {
        Ok(conn) => conn,
        Err(err) => bail!("CONTEXT.RUN error creating SQLITE connection: {}", err),
    };
    log::debug!("Compiling SQL: {}", &sql_str);
    let mut stmt = match conn.prepare(&sql_str) {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("CONTEXT.RUN error compile SQL statement: {}", err);
        }
    };
    let column_n = stmt.column_count();
    let mut columns_map: HashMap<usize, String> = HashMap::new();
    for c in 0..column_n {
        let name = stmt.column_name(c).expect("valid column index");
        columns_map.insert(c, name.to_string());
    }
    match stmt.query([]) {
        Ok(mut rows) => {
            let mut row_count: usize = 0;
            loop {
                match rows.next() {
                    Ok(Some(row)) => {
                        if row_count < skip_first as usize {
                            row_count += 1;
                            continue;
                        }
                        let mut bund_row = Value::dict();
                        for c in 0..column_n {
                            let value = row.get_ref(c);
                            let name = columns_map.get(&c).unwrap();
                            match value {
                                Ok(value) => {
                                    match value {
                                        ValueRef::Null => {
                                            bund_row = bund_row.set(name, Value::nodata());
                                        }
                                        ValueRef::Integer(i) => {
                                            bund_row = bund_row.set(name, Value::from_int(i));
                                        }
                                        ValueRef::Real(f) => {
                                            bund_row = bund_row.set(name, Value::from_float(f));
                                        }
                                        ValueRef::Text(s) => {
                                            let string_data = match std::str::from_utf8(s) {
                                                Ok(string_data) => string_data,
                                                Err(err) => bail!("CONTEXT.RUN error converting string data: {}", err),
                                            };
                                            bund_row = bund_row.set(name, Value::from_string(string_data));
                                        }
                                        ValueRef::Blob(b) => {
                                            bund_row = bund_row.set(name, Value::from_binary(b.to_vec())?);
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("CONTEXT.RUN error get field: {}", err);
                                }
                            }
                        }
                        vm.stack.push(bund_row.clone());
                        let ret = vm.lambda_eval(lambda_val.clone());
                        match ret {
                            Ok(_) => {},
                            Err(err) => bail!("SQLITE3 processing lambda returns: {}", err),
                        };
                        row_count += 1;
                    }
                    Ok(None) => break,
                    Err(err) => {
                        bail!("CONTEXT.RUN error getting row: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            bail!("CONTEXT.RUN: Error execution of SQL query: {}", err);
        }
    }
    Ok(vm)
}
