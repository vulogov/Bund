extern crate log;
use rust_dynamic::value::Value;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_bootstrap<'a>(vm: &'a mut VM, conn: &mut Connection, name: String) -> Result<&'a mut VM, Error> {
    let mut stmt = match conn.prepare("SELECT script FROM BOOTSTRAP WHERE name=?1") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling BOOTSTRAP select: {:?}", err);
        }
    };
    match stmt.query([name.clone(),]) {
        Ok(mut rows) => {
            loop {
                match rows.next() {
                    Ok(Some(row)) => {
                        let script: String = match row.get(0) {
                            Ok(script) => script,
                            Err(err) => {
                                bail!("Error getting script {}: {}", &name, err);
                            }
                        };
                        let _ = vm.stack.push(Value::from_string(script));
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::debug!("Error getting SCRIPT row: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            bail!("Error performing SCRIPT select: {:?}", err);
        }
    }
    Ok(vm)
}

pub fn save_bootstrap<'a>(vm: &'a mut VM, conn: &mut Connection, name: String, snippet: String) -> Result<&'a mut VM, Error> {
    match conn.execute("DELETE FROM BOOTSTRAP WHERE name=?1", (name.clone(),)) {
        Ok(_) => {},
        Err(err) => {
            bail!("Deleting previous boostrap entry returns: {}", err);
        }
    }
    match conn.execute("INSERT INTO BOOTSTRAP (name, script) VALUES (?1, ?2)", (name.clone(), snippet)) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating boostrap entry returns: {}", err);
        }
    }
    Ok(vm)
}

pub fn init<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS BOOTSTRAP", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping bootstrap table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS BOOTSTRAP (
            name    TEXT PRIMARY KEY,
            script  BLOB
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating bootstrap table returns: {}", err);
        }
    }
    Ok(vm)
}
