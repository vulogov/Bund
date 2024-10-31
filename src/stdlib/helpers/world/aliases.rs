extern crate log;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_aliases<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    let mut stmt = match conn.prepare("SELECT name, alias FROM ALIASES") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling ALIASES select: {:?}", err);
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
                                bail!("Error getting name of alias: {}", err);
                            }
                        };
                        let alias: String = match row.get(1) {
                            Ok(name) => name,
                            Err(err) => {
                                bail!("Error getting alias of alias {}: {}", &name, err);
                            }
                        };
                        match vm.register_alias(name.clone(), alias.clone()) {
                            Ok(_) => {},
                            Err(err) => {
                                bail!("Error geristering alias: {}", err);
                            }
                        }
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::debug!("Error getting ALIAS row: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            bail!("Error performing ALIASES select: {:?}", err);
        }
    }
    Ok(vm)
}

pub fn save_aliases<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS ALIASES", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping aliases table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS ALIASES (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            alias   TEXT NOT NULL
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating aliases table returns: {}", err);
        }
    }
    for (k, v) in vm.name_mapping.iter() {
        match conn.execute("INSERT INTO ALIASES (name, alias) VALUES (?1, ?2)", (k.clone(), v.clone())) {
            Ok(_) => {},
            Err(err) => {
                bail!("Saving alias returns: {}", err);
            }
        }
    }
    Ok(vm)
}
