extern crate log;
use crate::stdlib::functions::internaldb;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value;
use crate::stdlib::{functions};
use duckdb::{params};
use duckdb::types::Value;
use easy_error::{Error, bail};

const DOCSTORE_INIT: &str = r###"
CREATE TABLE DOCUMENTS (
    name            VARCHAR,
    documentid      VARCHAR PRIMARY KEY,
    originaldocid   VARCHAR,
    data            VARCHAR,
    embeddings      FLOAT[]
);
"###;

const DOCSTORE_INIT_IX1: &str = r###"
CREATE INDEX name_idx ON DOCUMENTS(name);
"###;

const DOCSTORE_DROP: &str = r###"
DROP TABLE DOCUMENTS;
"###;

const DOCSTORE_DROP_IX1: &str = r###"
DROP INDEX name_idx;
"###;

#[time_graph::instrument]
pub fn internaldb_store_document(document: value::Value) -> Result<(), Error> {
    let db = match internaldb::DB.lock() {
        Ok(db) => db,
        Err(err) => bail!("INTERNALDB.EXECUTE: getting reference to internal database returns error: {}", err),
    };
    let document_data = match functions::ai::document::local_embed_document(document) {
        Ok(document) => document,
        Err(err) => {
            drop(db);
            bail!("DOCSTORE.STORE can not run embedding: {}", err)
        },
    };
    // for doc in document_data {
    //     let mut emb: Vec<f32> = Vec::new();
    //     emb.push(3.14);
    //     let mut stmt = db.prepare("INSERT INTO DOCUMENTS (name, embeddings) VALUES (?, ?)").unwrap();
    //     stmt.execute(params!["Joe Smith", emb]).unwrap();
    // }
    drop(db);
    Ok(())
}

#[time_graph::instrument]
pub fn stdlib_internaldb_docstore_init(vm: &mut VM) -> Result<&mut VM, Error> {
    match internaldb::execute::internaldb_execute_query(DOCSTORE_INIT.to_string()) {
        Ok(_) => {},
        Err(err) => bail!("DOCSTORE.INIT returned an error: {}", err),
    };
    match internaldb::execute::internaldb_execute_query(DOCSTORE_INIT_IX1.to_string()) {
        Ok(_) => {},
        Err(err) => bail!("DOCSTORE.INIT returned an error: {}", err),
    };
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_internaldb_docstore_drop(vm: &mut VM) -> Result<&mut VM, Error> {
    match internaldb::execute::internaldb_execute_query(DOCSTORE_DROP_IX1.to_string()) {
        Ok(_) => {},
        Err(err) => bail!("DOCSTORE.DROP returned an error: {}", err),
    };
    match internaldb::execute::internaldb_execute_query(DOCSTORE_DROP.to_string()) {
        Ok(_) => {},
        Err(err) => bail!("DOCSTORE.DROP returned an error: {}", err),
    };
    Ok(vm)
}
