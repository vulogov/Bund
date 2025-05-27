extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use uuid::Uuid;
use ulid::Ulid;
use tokenizers::Tokenizer;
use text_splitter::{ChunkConfig, TextSplitter};
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};

const DEFAULT_DOCUMENT_TEMPLATE: &str = r###"
NAME: {name}

{.data}

documentid: {.documentid}
"###;

#[time_graph::instrument]
fn construct_full_document(value: Value) -> Value {
    let mut full_document: String = "".to_string();
    for chunk in value {
        let chunk_str = match chunk.conv(STRING) {
            Ok(chunk_str) => match chunk_str.cast_string() {
                Ok(chunk_str) => chunk_str,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        full_document = full_document + &chunk_str;
    }
    Value::from_str(&full_document)
}

#[time_graph::instrument]
fn split_document(mut value: Value) -> Result<Value, Error> {
    match oop::value_class::locate_value_in_object(".data".to_string(), value.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                LIST => {
                    let full_document = construct_full_document(data_object);
                    let max_tokens: usize = match value.get(".token_max") {
                        Ok(max_tokens) => match max_tokens.cast_int() {
                            Ok(max_tokens) => max_tokens as usize,
                            Err(_) => 512 as usize,
                        },
                        Err(_) => 512 as usize,
                    };
                    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None).unwrap();
                    let splitter = TextSplitter::new(ChunkConfig::new(max_tokens).with_sizer(tokenizer));
                    let full_document_str = full_document.cast_string().unwrap();
                    let chunks = splitter.chunks(&full_document_str);
                    let mut res = Value::list();
                    for c in chunks {
                        res = res.push(Value::from_string(c));
                    }
                    value = oop::value_class::set_value_in_object(".data".to_string(), value.clone(), res);
                }
                _ => bail!("DOCUMENT: WRAPPED TYPE IS NOT SUPPORTED IN SPLIT: {}", &data_object.type_name()),
            }
        }
        None => bail!("DOCUMENT: NO WRAPPED DATA WAS FOUND FOR SPLIT"),
    }
    Ok(value)
}

#[time_graph::instrument]
fn export_document(value: Value) -> Result<Value, Error> {
    let name = match value.get("name") {
        Ok(name) => match name.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("DOCUMENT: ERROR CASTING DOCUMENT NAME: {}", err),
        },
        Err(_) => Ulid::new().to_string(),
    };
    let documentid = match value.get(".documentid") {
        Ok(documentid) => documentid,
        Err(err) => bail!("DOCUMENT: ERROR GETTING DOCUMENT ID: {}", err),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                LIST => {
                    let mut res = Value::list();
                    let n: usize = 0;
                    for c in data_object {
                        let mut chunk = Value::dict();
                        chunk = chunk.set("name", Value::from_string(format!("[{}] {}", n, &name)));
                        chunk = chunk.set(".data", c);
                        chunk = chunk.set(".documentid", Value::from_string(Uuid::new_v4().to_string()));
                        chunk = chunk.set(".original_documentid", documentid.clone());
                        res = res.push(chunk);
                    }
                    return Ok(res);
                }
                _ => bail!("DOCUMENT: WRAPPED TYPE IS NOT SUPPORTED IN EXPORT: {}", &data_object.type_name()),
            }
        }
        None => bail!("DOCUMENT: NO WRAPPED DATA WAS FOUND FOR EXPORT"),
    }
}

#[time_graph::instrument]
fn wrap_embeddings(e: Vec<f32>) -> Value {
    let mut res = Value::list();
    for v in e {
        res = res.push(Value::from_float(v as f64))
    }
    res
}

#[time_graph::instrument]
fn local_embed_document(value: Value) -> Result<Value, Error> {
    let name = match value.get("name") {
        Ok(name) => match name.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("DOCUMENT: ERROR CASTING DOCUMENT NAME: {}", err),
        },
        Err(_) => Ulid::new().to_string(),
    };
    let documentid = match value.get(".documentid") {
        Ok(documentid) => documentid,
        Err(err) => bail!("DOCUMENT: ERROR GETTING DOCUMENT ID: {}", err),
    };
    let model_name: EmbeddingModel = match value.get(".embedding_model") {
        Ok(name) => match name.cast_string() {
            Ok(name) => match name.as_str() {
                "nomic" => EmbeddingModel::NomicEmbedTextV15,
                _ => bail!("DOCUMENT: embedding not supported: {}", &name),
            },
            Err(err) => bail!("DOCUMENT: ERROR CASTING DOCUMENT EMBEDDING: {}", err),
        },
        Err(err) => bail!("DOCUMENT: ERROR GETTING DOCUMENT EMBEDDING: {}", err),
    };
    let model = match TextEmbedding::try_new(
       InitOptions::new(model_name).with_show_download_progress(false),
    ) {
       Ok(model) => model,
       Err(err) => bail!("DOCUMENT: ERROR CREATING EMBEDDING MODEL: {}", err),
   };
    match oop::value_class::locate_value_in_object(".data".to_string(), value.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                LIST => {
                    let mut res = Value::list();
                    let mut n: usize = 0;
                    for c in data_object.into_iter() {
                        let mut docs: Vec<String> = Vec::new();
                        docs.push(c.cast_string().unwrap());
                        let embeddings = match model.embed(docs, None) {
                            Ok(embeddings) => embeddings,
                            Err(err) => bail!("DOCUMENT: ERROR GENERATE EMBEDDINGS: {}", err),
                        };
                        let mut chunk = Value::dict();
                        chunk = chunk.set("name", Value::from_string(format!("[{}] {}", n, &name)));
                        chunk = chunk.set(".data", c);
                        chunk = chunk.set(".documentid", Value::from_string(Uuid::new_v4().to_string()));
                        chunk = chunk.set(".original_documentid", documentid.clone());
                        chunk = chunk.set("embedings", wrap_embeddings(embeddings[0].clone()));
                        res = res.push(chunk);
                        n = n + 1;
                    }
                    return Ok(res);
                }
                _ => bail!("DOCUMENT: WRAPPED TYPE IS NOT SUPPORTED IN EXPORT: {}", &data_object.type_name()),
            }
        }
        None => bail!("DOCUMENT: NO WRAPPED DATA WAS FOUND FOR EXPORT"),
    }
}

#[time_graph::instrument]
fn register_method_document_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DOCUMENT: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                STRING => {
                    let mut data = Value::list();
                    data = data.push(data_object);
                    value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object.clone(), data);
                    value_object = match split_document(value_object) {
                        Ok(value_object) => value_object,
                        Err(err) => bail!("DOCUMENT: ERROR SPLITTING DOCUMENT: {}", err),
                    };
                }
                _ => bail!("DOCUMENT: WRAPPED TYPE IS NOT SUPPORTED: {}", &data_object.type_name()),
            }
        }
        None => bail!("DOCUMENT: NO WRAPPED DATA WAS FOUND FOR INIT"),
    }
    value_object = value_object.set("name".to_string(), Value::from_string(Ulid::new().to_string()));
    value_object = value_object.set(".documentid".to_string(), Value::from_string(Uuid::new_v4().to_string()));
    value_object = value_object.set(".embedding_model".to_string(), Value::from_string("nomic"));
    vm.stack.push(value_object);
    Ok(vm)
}

#[time_graph::instrument]
fn register_method_document_embeddings(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DOCUMENT: stack is empty"),
    };
    let embeddings = match local_embed_document(value_object.clone()) {
        Ok(embeddings) => embeddings,
        Err(err) => bail!("DOCUMENT: ERROR GENERATE EMBEDDINGS FOR DOCUMENT: {}", err),
    };
    vm.stack.push(value_object);
    vm.stack.push(embeddings);
    Ok(vm)
}

#[time_graph::instrument]
fn register_method_document_export(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DOCUMENT: stack is empty"),
    };
    let exported_document = match export_document(value_object.clone()) {
        Ok(exported_document) => exported_document,
        Err(err) => bail!("DOCUMENT: ERROR EXPORTING DOCUMENT: {}", err),
    };
    vm.stack.push(value_object);
    vm.stack.push(exported_document);
    Ok(vm)
}

#[time_graph::instrument]
fn register_method_document_len(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DOCUMENT: stack is empty"),
    };
    let mut n: usize = 0;
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                LIST => {
                    for c in data_object {
                        n = n + c.len();
                    }
                }
                _ => bail!("DOCUMENT: WRAPPED TYPE IS NOT SUPPORTED IN LEN: {}", &data_object.type_name()),
            }
        }
        None => bail!("DOCUMENT: NO WRAPPED DATA WAS FOUND FOR LEN"),
    }
    vm.stack.push(value_object);
    vm.stack.push(Value::from_int(n as i64));
    Ok(vm)
}

fn register_document(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".document_init".to_string(), register_method_document_init);
    let _ = vm.register_method(".document_export".to_string(), register_method_document_export);
    let _ = vm.register_method(".document_embeddings".to_string(), register_method_document_embeddings);
    let _ = vm.register_method(".document_len".to_string(), register_method_document_len);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("DOCUMENT"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".document_init".to_string(), Vec::new()));
    obj_class = obj_class.set("export".to_string(), Value::ptr(".document_export".to_string(), Vec::new()));
    obj_class = obj_class.set("embeddings".to_string(), Value::ptr(".document_embeddings".to_string(), Vec::new()));
    obj_class = obj_class.set("len".to_string(), Value::ptr(".document_len".to_string(), Vec::new()));
    //
    // Registed default attributes
    //
    obj_class = obj_class.set(".template".to_string(), Value::from_string(DEFAULT_DOCUMENT_TEMPLATE));
    obj_class = obj_class.set("name".to_string(), Value::from_string(""));
    obj_class = obj_class.set(".documentid".to_string(), Value::from_string(""));
    obj_class = obj_class.set(".chunk_min".to_string(), Value::from_int(128 as i64));
    obj_class = obj_class.set(".chunk_max".to_string(), Value::from_int(256 as i64));
    obj_class = obj_class.set(".token_max".to_string(), Value::from_int(512 as i64));
    vm.register_class("DOCUMENT".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    match register_document(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing DOCUMENT base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
