extern crate log;
use rust_dynamic::value::Value;
use rusqlite::{Connection};
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

pub fn load_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    let stacks = get_stacks(vm, conn);
    let mut stmt = match conn.prepare("SELECT value FROM STACK_DATA WHERE name=?1 ORDER BY pos") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling SELECT for stack data: {}", err);
        }
    };
    match stacks {
        Ok(stacks) => {
            for s in stacks {
                let _ = match stmt.query([s.clone()]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    let raw_value: Vec<u8> = match row.get(0) {
                                        Ok(raw_value) => raw_value,
                                        Err(err) => {
                                            bail!("Error getting value of stack: {}", err);
                                        }
                                    };
                                    match Value::from_binary(raw_value) {
                                        Ok(value) => {
                                            vm.stack.push_to_stack(s.clone(), value);
                                        }
                                        Err(err) => {
                                            bail!("Extraction of value returns: {}", err);
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
                        bail!("Error executing SELECT for stacks: {}", err);
                    }
                };
            }
        }
        Err(err) => {
            bail!("Error getting list of stacks: {}", err);
        }
    }
    Ok(vm)
}

pub fn get_stacks<'a>(_vm: &'a mut VM, conn: &mut Connection) -> Result<Vec<String>, Error> {
    let mut res: Vec<String> = Vec::new();
    let mut stmt = match conn.prepare("SELECT name FROM STACKS") {
        Ok(stmt) => stmt,
        Err(err) => {
            bail!("Error compiling SELECT for stacks: {}", err);
        }
    };
    let _ = match stmt.query([]) {
        Ok(mut rows) => {
            loop {
                match rows.next() {
                    Ok(Some(row)) => {
                        let name: String = match row.get(0) {
                            Ok(name) => name,
                            Err(err) => {
                                bail!("Error getting name of stack: {}", err);
                            }
                        };
                        res.push(name.clone());
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::debug!("Error getting STACKS row: {}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            bail!("Error executing SELECT for stacks: {}", err);
        }
    };
    Ok(res)
}

fn save_data_for_stack<'a>(vm: &'a mut VM, conn: &mut Connection, name: String) -> Result<&'a mut VM, Error> {
    if vm.stack.stack.contains_key(&name) {
        let stack = &vm.stack.stack[&name];
        let mut pos = 0;
        for v in stack.stack.iter() {
            match v.to_binary() {
                Ok(blob) => {
                    match conn.execute("INSERT INTO STACK_DATA (name, pos, value) VALUES (?1, ?2, ?3)", (name.clone(), pos, blob)) {
                        Ok(_) => {},
                        Err(err) => {
                            bail!("Saving stack data returns: {}", err);
                        }
                    }
                }
                Err(err) => {
                    bail!("Error converting data to blob: {}", err);
                }
            }
            pos += 1;
        }
        return Ok(vm);
    } else {
        bail!("Stack not found for SAVE operation");
    }
}

fn save_stack_data<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS STACK_DATA", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping stack_data table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS STACK_DATA (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            pos     INTEGER,
            value   BLOB
        )",
        ()) {
        Ok(_) => {
            match conn.execute(
                "DROP INDEX IF EXISTS STACK_DATA.STACK_AND_POS",
            ()) {
                Ok(_) => {
                    match conn.execute(
                        "CREATE UNIQUE INDEX IF NOT EXISTS STACK_AND_POS ON STACK_DATA(name,pos)",
                    ()) {
                        Ok(_) => {
                            match get_stacks(vm, conn) {
                                Ok(stacks) => {
                                    for s in stacks {
                                        match save_data_for_stack(vm, conn, s) {
                                            Ok(_) => {},
                                            Err(err) => {
                                                bail!("Saving stack data returns: {}", err);
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("Error getting list of stacks: {}", err);
                                }
                            }
                        }
                        Err(err) => {
                            bail!("Creating stack_data index table returns: {}", err);
                        }
                    }

                }
                Err(err) => {
                    bail!("Dropping stack_data index table returns: {}", err);
                }
            }
        },
        Err(err) => {
            bail!("Creating stack_data table returns: {}", err);
        }
    }
    Ok(vm)
}

pub fn save_stacks<'a>(vm: &'a mut VM, conn: &mut Connection) -> Result<&'a mut VM, Error> {
    match conn.execute("DROP TABLE IF EXISTS STACKS", ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Dropping stacks table returns: {}", err);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS STACKS (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL
        )",
        ()) {
        Ok(_) => {},
        Err(err) => {
            bail!("Creating stacks table returns: {}", err);
        }
    }
    for n in vm.stack.stacks.iter() {
        match conn.execute("INSERT INTO STACKS (name) VALUES (?1)", (n.clone(),)) {
            Ok(_) => {},
            Err(err) => {
                bail!("Saving stacks returns: {}", err);
            }
        }
    }
    save_stack_data(vm, conn)
}
