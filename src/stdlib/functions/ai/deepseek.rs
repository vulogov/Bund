extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use curl::easy::List;
use serde_json::json;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};


pub fn stdlib_deepseek_token(vm: &mut VM) -> Result<&mut VM, Error> {
    let cli = cmd::CLI.lock();
    let token = match &cli.as_ref().unwrap().ai.deepseek_token {
        Some(token) => token,
        None => {
            drop(cli);
            bail!("DEEPSEEK::TOKEN not provided")
        },
    };
    vm.stack.push(Value::from_string(token));
    drop(cli);
    Ok(vm)
}

fn register_method_deepseek_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DEEPSEEK: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                MAP => {
                    let token = match data_object.get("api_token") {
                        Ok(token) => match token.cast_string() {
                            Ok(token) => token,
                            Err(err) => bail!("DEEPSEEK: error casting address: {}", err),
                        },
                        Err(err) => bail!("DEEPSEEK: error getting token: {}", err),
                    };
                    value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&token));
                    vm.stack.push(value_object);
                }
                STRING => {
                    let data = data_object.cast_string().unwrap();
                    let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&data));
                    vm.stack.push(new_value_object);
                }
                _ => bail!("DEEPSEEK: WRAPPED TYPE IS NOT SUPPORTED: {}", &data_object.type_name()),
            }
        }
        None => bail!("DEEPSEEK: NO WRAPPED DATA WAS FOUND FOR INIT"),
    }
    Ok(vm)
}

fn register_method_deepseek_ask(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DEEPSEEK::ASK stack is empty"),
    };
    let token = match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(api_token) => match api_token.cast_string() {
            Ok(api_token) => api_token,
            Err(err) => bail!("DEEPSEEK::ASK error casting API token: {}", err),
        },
        None => bail!("DEEPSEEK::ASK token not found"),
    };
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("DEEPSEEK::ASK casting request returns: {}", err),
        },
        None => bail!("DEEPSEEK::ASK: NO DATA #2"),
    };

    let full_url = "https://api.deepseek.com/chat/completions".to_string();

    let mut messages: Vec<helpers::json_api::Message> = vec![];

    messages.push(helpers::json_api::Message {
        role: String::from("user"),
        content: msg,
    });

    let payload = json!({
        "model": "deepseek-chat",
        "messages": &messages,
        "stream": false,
    });

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    headers.append(&format!("Authorization: Bearer {}", token).to_string()).unwrap();

    let json_data: serde_json::Value = match helpers::json_api::json_api_post(full_url, headers, payload) {
        Some(json_value) => json_value,
        None => bail!("DEEPSEEK::ASK did not get any meaningful data"),
    };

    let json_choices: Vec<serde_json::Value> = match json_data.get("choices") {
        Some(json_choices) => match json_choices.as_array() {
            Some(json_choices) => json_choices.to_vec(),
            None => bail!("DEEPSEEK::ASK did not get any meaningful array"),
        },
        None => bail!("DEEPSEEK::ASK did not get any meaningful choices"),
    };

    let mut res: String = "".to_string();

    for c in json_choices.iter() {
        let json_message = match c.get("message") {
            Some(message) => match message.get("content") {
                Some(content) => match content.as_str() {
                    Some(content) => content,
                    None => bail!("DEEPSEEK::ASK can not parse JSON"),
                },
                None => bail!("DEEPSEEK::ASK Deepseek did not returned a content"),
            },
            None => bail!("DEEPSEEK::ASK did not returned a message"),
        };
        res.push_str(json_message);
        res.push_str("\n");
    }

    vm.stack.push(value_object);
    vm.stack.push(Value::from_string(res));

    Ok(vm)
}

fn register_deepseek(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".deepseek_init".to_string(), register_method_deepseek_init);
    let _ = vm.register_method(".deepseek_ask".to_string(), register_method_deepseek_ask);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    super_class = super_class.push(Value::from_string("Display"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("DEEPSEEK"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".deepseek_init".to_string(), Vec::new()));
    obj_class = obj_class.set("ask".to_string(), Value::ptr(".deepseek_ask".to_string(), Vec::new()));
    vm.register_class("DEEPSEEK".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    match register_deepseek(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registering DEEPSEEK base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    let _ = bc.vm.register_inline("deepseek.token".to_string(), stdlib_deepseek_token);

    drop(bc);
}
