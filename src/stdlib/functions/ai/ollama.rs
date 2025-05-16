extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::types::*;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use rust_dynamic::value::Value;
use serde::{Serialize, Deserialize};
use serde_json::json;
use curl::easy::{Easy2, Handler, WriteError};
use easy_error::{Error, bail};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}


fn register_method_ollama_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("OLLAMA: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                MAP => {
                    let addr = match data_object.get("address") {
                        Ok(addr) => match addr.cast_string() {
                            Ok(addr) => addr,
                            Err(err) => bail!("OLLAMA: error casting address: {}", err),
                        },
                        Err(_) => "127.0.0.1".to_string(),
                    };
                    let url = format!("http://{}", &addr);
                    value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&addr));
                    value_object = value_object.set("url", Value::from_string(&url));
                    log::debug!("OLLAMA object with URL: {}", &url);
                    vm.stack.push(value_object);
                }
                STRING => {
                    let data = data_object.cast_string().unwrap();
                    let url = format!("http://{}", &data);
                    let mut new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&data));
                    new_value_object = new_value_object.set("url", Value::from_string(&url));
                    log::debug!("OLLAMA object with URL: {}", &url);
                    vm.stack.push(new_value_object);
                }
                _ => bail!("OLLAMA: WRAPPED TYPE IS NOT SUPPORTED: {}", &data_object.type_name()),
            }
        }
        None => bail!("OLLAMA: NO WRAPPED DATA WAS FOUND FOR INIT"),
    }
    Ok(vm)
}


fn register_method_ollama_ask(vm: &mut VM) -> Result<&mut VM, Error> {
    #[derive(Debug)]
    struct Collector(Vec<u8>);

    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("OLLAMA::ASK stack is empty"),
    };

    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("OLLAMA::ASK casting request returns: {}", err),
        },
        None => bail!("OLLAMA::ASK: NO DATA #1"),
    };

    let url = match value_object.get("url") {
            Ok(url) => match url.cast_string() {
                Ok(url) => url,
                Err(err) => bail!("OLLAMA::ASK error casting OLLAMA URL: {}", err),
            },
            Err(err) => bail!("OLLAMA::ASK can not discover OLLAMA URL: {}", err),
    };

    let full_url = format!("{}/api/chat", url);

    let mut messages: Vec<Message> = vec![];

    messages.push(Message {
        role: String::from("user"),
        content: msg,
    });

    let payload = json!({
        "model": "llama2",
        "messages": &messages,
        "stream": false,
    });

    impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    let _ = easy.useragent("BUND");

    easy.post(true).unwrap();
    easy.url(&full_url).unwrap();
    easy.post_fields_copy(format!("{}", &payload).as_bytes()).unwrap();
    match easy.perform() {
        Ok(_) => {},
        Err(err) => bail!("OLLAMA::ASK perform returns: {}", err),
    }


    let contents = easy.get_ref();
    let raw_data = String::from_utf8_lossy(&contents.0).to_string();

    let json_data: serde_json::Value = match serde_json::from_str(&raw_data) {
        Ok(json_data) => json_data,
        Err(err) => bail!("OLLAMA::ASK can produce JSON: {}", err),
    };

    let json_message = match json_data.get("message") {
        Some(message) => match message.get("content") {
            Some(content) => match content.as_str() {
                Some(content) => content,
                None => bail!("OLLAMA::ASK can not parse JSON"),
            },
            None => bail!("OLLAMA::ASK Ollama did not returned a content"),
        },
        None => bail!("OLLAMA::ASK Ollama did not returned a message"),
    };

    vm.stack.push(value_object);
    vm.stack.push(Value::from_string(json_message));

    Ok(vm)
}

fn register_ollama(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".ollama_init".to_string(), register_method_ollama_init);
    let _ = vm.register_method(".ollama_ask".to_string(), register_method_ollama_ask);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    super_class = super_class.push(Value::from_string("Display"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("OLLAMA"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".ollama_init".to_string(), Vec::new()));
    obj_class = obj_class.set("ask".to_string(), Value::ptr(".ollama_ask".to_string(), Vec::new()));
    vm.register_class("OLLAMA".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    match register_ollama(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing OLLAMA base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
