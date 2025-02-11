extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::stdlib::helpers;
use polars_core::prelude::*;
use polars_io::prelude::*;
use easy_error::{Error, bail};

pub fn stdlib_conditional_csv(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for curry");
    }
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("CONTEXT: No context name discovered on the stack"),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT: Error name casting: {}", err),
    };
    if ! helpers::filesystem_helper::filesystem_is_file(name.clone()) {
        bail!("CSV: csv file not found: {}", &name);
    }
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("csv"));
    res = res.set("name", Value::from_string(name.clone()));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    let name_val = match value.get("name") {
        Ok(name_val) => name_val,
        Err(err) => bail!("CONTEXT.RUN: getting CSV file name returns error: {}", err),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT.RUN: casting CSV file name returns error: {}", err),
    };
    let column_val = match value.get("column") {
        Ok(column_val) => Some(column_val),
        Err(_) => None,
    };
    let header_val = match value.get("is_header") {
        Ok(header_val) => header_val,
        Err(_) => Value::make_true(),
    };
    let is_header = match header_val.cast_bool() {
        Ok(is_header) => is_header,
        Err(err) => bail!("CONTEXT.RUN: casting CSV is_header returns error: {}", err),
    };
    log::debug!("CONTEXT.RUN: processing CSV file name: {}", &name);
    if ! helpers::filesystem_helper::filesystem_is_file(name.clone()) {
        bail!("CSV: csv file not found: {}", &name);
    }
    let lambda_val = match value.get("lambda") {
        Ok(lambda_val) => lambda_val,
        Err(_) => Value::lambda(),
    };
    match column_val {
        Some(column_val) => {
            match CsvReadOptions::default().with_has_header(is_header).try_into_reader_with_file_path(Some(name.into())) {
                Ok(df_unfinished) => {
                    match df_unfinished.finish() {
                        Ok(df) => {
                            let column_name = match column_val.cast_string() {
                                Ok(column_name) => column_name,
                                Err(err) => bail!("CONTEXT.RUN error casting column name: {}", err),
                            };
                            match df.select([column_name.clone()]) {
                                Ok(df_column) => {
                                    let mut res = Value::list();
                                    for i in  0..df_column.height() {
                                        match df_column.get(i) {
                                            Some(df_row) => {
                                                if df_row.len() > 0 {
                                                    match df_row[0] {
                                                        AnyValue::String(str_value) => {
                                                            res = res.push(Value::from_string(str_value));
                                                        }
                                                        AnyValue::Int64(int_value) => {
                                                            res = res.push(Value::from_int(int_value));
                                                        }
                                                        AnyValue::Float64(float_value) => {
                                                            res = res.push(Value::from_float(float_value));
                                                        }
                                                        AnyValue::Boolean(bool_value) => {
                                                            res = res.push(Value::from_bool(bool_value));
                                                        }
                                                        _ => continue,
                                                    }
                                                }
                                            }
                                            None => {
                                                break;
                                            }
                                        }
                                    }
                                    vm.stack.push(res);
                                    let ret = vm.lambda_eval(lambda_val.clone());
                                    match ret {
                                        Ok(_) => {},
                                        Err(err) => bail!("CSV processing lambda returns: {}", err),
                                    };
                                }
                                Err(err) => {
                                    bail!("CONTEXT.RUN: Error selecting column {}: {}", &column_name, err);
                                }
                            }
                        }
                        Err(err) => {
                            bail!("CONTEXT.RUN: Error finishing dataframe: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("CONTEXT.RUN: Error creating dataframe: {}", err);
                }
            }
        }
        None => {
            match CsvReadOptions::default().with_has_header(is_header).try_into_reader_with_file_path(Some(name.into())) {
                Ok(df_unfinished) => {
                    match df_unfinished.finish() {
                        Ok(df) => {
                            for i in  0..df.height() {
                                match df.get(i) {
                                    Some(df_row) => {
                                        let mut row = Value::list();
                                        for f in df_row {
                                            match f {
                                                AnyValue::String(str_value) => {
                                                    row = row.push(Value::from_string(str_value));
                                                }
                                                AnyValue::Int64(int_value) => {
                                                    row = row.push(Value::from_int(int_value));
                                                }
                                                AnyValue::Float64(float_value) => {
                                                    row = row.push(Value::from_float(float_value));
                                                }
                                                AnyValue::Boolean(bool_value) => {
                                                    row = row.push(Value::from_bool(bool_value));
                                                }
                                                _ => continue,
                                            }
                                        }
                                        vm.stack.push(row);
                                        let ret = vm.lambda_eval(lambda_val.clone());
                                        match ret {
                                            Ok(_) => {},
                                            Err(err) => bail!("CSV processing lambda returns: {}", err),
                                        };
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            bail!("CONTEXT.RUN: Error finishing dataframe: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("CONTEXT.RUN: Error creating dataframe: {}", err);
                }
            }
        }
    }

    Ok(vm)
}
