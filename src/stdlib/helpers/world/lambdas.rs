extern crate log;
use rust_dynamic::value::Value;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_lambdas<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    let mut stmt = match conn.prepare("SELECT name, lambda FROM LAMBDAS") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling LAMBDAS select: {:?}", err);
        }
    };
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
                        let lambda: Vec<u8> = match row.get(1) {
                            Ok(name) => name,
                            Err(err) => {
                                bail!("Error getting body of lambda: {}", err);
                            }
                        };
                        log::debug!("Loading lambda {} len()={}", &name, &lambda.len());
                        match Value::from_binary(lambda) {
                            Ok(lambda_value) => {
                                match vm.register_lambda(name, lambda_value) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        bail!("Error registering the lambda: {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                bail!("Error recovering the body of lambda: {}", err);
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::debug!("Error getting LAMBDA row: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            bail!("Error performing LAMBDAS select: {:?}", err);
        }
    }
    Ok(vm)
}

pub fn save_lambdas<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS LAMBDAS", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping lambdas table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS LAMBDAS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            lambda  BLOB
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating lambdas table returns: {}", err);
        }
    }
    for (n,l) in vm.lambdas.iter() {
        match l.to_binary() {
            Ok(lambda_data) => {
                log::debug!("Generated lambda for store: {} len()={}", &n, &lambda_data.len());
                match conn.execute("INSERT INTO LAMBDAS (name, lambda) VALUES (?1, ?2)", (n.clone(), lambda_data)) {
                    Ok(_) => {},
                    Err(err) => {
                        bail!("Saving lambda returns: {}", err);
                    }
                }
            }
            Err(err) => {
                bail!("Error compiling lambda body: {}", err);
            }
        }
    }
    Ok(vm)
}
