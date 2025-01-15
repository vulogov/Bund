extern crate log;
use rusqlite::{Connection};
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};


#[time_graph::instrument]
pub fn load_model<'a>(vm: &'a mut VM, conn: &mut Connection, model_name: String) -> Result<&'a mut VM, Error> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS MODELS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            model  BLOB
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating models table returns: {}", err);
        }
    }
    let mut stmt = match conn.prepare("SELECT name, model FROM MODELS") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling MODELS select: {:?}", err);
        }
    };
    let mut c = 0;
    match stmt.query([]) {
        Ok(mut rows) => {
            loop {
                match rows.next() {
                    Ok(Some(row)) => {
                        let name: String = match row.get(0) {
                            Ok(name) => name,
                            Err(err) => {
                                bail!("Error getting name of lambda: {}", err);
                            }
                        };
                        let model: Vec<u8> = match row.get(1) {
                            Ok(name) => name,
                            Err(err) => {
                                bail!("Error getting body of the model: {}", err);
                            }
                        };
                        log::debug!("Loading model {} len()={}", &name, &model.len());
                        match Value::from_binary(model) {
                            Ok(model_value) => {
                                vm.stack.push(model_value);
                                c = c + 1;
                            }
                            Err(err) => {
                                bail!("Error recovering the body of the model: {}", err);
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::debug!("Error getting MODEL row: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            bail!("Error performing MODEL select: {:?}", err);
        }
    }
    if c == 0 {
        bail!("Error loading model. None found with that name: {}", &model_name);
    }
    Ok(vm)
}


#[time_graph::instrument]
pub fn save_model<'a>(vm: &'a mut VM, conn: &mut Connection, model_name: String, model: Value) -> Result<&'a mut VM, Error> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS MODELS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            model  BLOB
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating models table returns: {}", err);
        }
    }
    match model.to_binary() {
        Ok(model_data) => {
            log::debug!("Generated model for store: {} len()={}", &model_name, &model_data.len());
            match conn.execute("INSERT INTO MODELS (name, model) VALUES (?1, ?2)", (model_name.clone(), model_data)) {
                Ok(_) => {},
                Err(err) => {
                    bail!("Saving model returns: {}", err);
                }
            }
        }
        Err(err) => {
            bail!("Error compiling mode: {}", err);
        }
    }
    Ok(vm)
}
