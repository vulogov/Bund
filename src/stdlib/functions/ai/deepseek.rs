extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};
use deepseek_api::Client;
use deepseek_api::{response::ModelType};

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
            Err(err) => bail!("DEEPSEEK::BALANCE error casting API token: {}", err),
        },
        None => bail!("DEEPSEEK::BALANCE token not found"),
    };
    let msg = match vm.stack.pull() {
        Some(msg) => match msg.cast_string() {
            Ok(msg) => msg,
            Err(err) => bail!("DEEPSEEK::ASK casting request returns: {}", err),
        },
        None => bail!("DEEPSEEK::ASK: NO DATA #2"),
    };
    let client = Client::new(&token);
    let mut completions = client.chat();
    let builder = completions
                    .chat_builder(vec![])
                    .use_model(ModelType::DeepSeekChat)
                    .append_user_message(&msg);
    let resp = completions.create(builder).unwrap().must_response();
    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        let ds_msg = match msg.message.as_ref() {
            Some(msg) => msg.content.clone(),
            None => bail!("DEEPSEEK::ASK referencing message request returns: None"),
        };
        resp_words.push(ds_msg);
    }

    let mut res = Value::from_string("");

    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| res = res.push(Value::from_string(x)));
    }
    vm.stack.push(value_object);
    vm.stack.push(res);
    Ok(vm)
}

fn register_method_deepseek_balance(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("DEEPSEEK::BALANCE stack is empty"),
    };
    let token = match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(api_token) => match api_token.cast_string() {
            Ok(api_token) => api_token,
            Err(err) => bail!("DEEPSEEK::BALANCE error casting API token: {}", err),
        },
        None => bail!("DEEPSEEK::BALANCE token not found"),
    };
    let client = Client::new(&token);
    let balance = match client.balance() {
        Ok(balance) => balance,
        Err(err) => bail!("DEEPSEEK::BALANCE error getting balance: {}", err),
    };
    value_object = value_object.set("available", Value::from_bool(balance.is_available));
    vm.stack.push(value_object);
    Ok(vm)
}

fn register_deepseek(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".deepseek_init".to_string(), register_method_deepseek_init);
    let _ = vm.register_method(".deepseek_balance".to_string(), register_method_deepseek_balance);
    let _ = vm.register_method(".deepseek_ask".to_string(), register_method_deepseek_ask);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    super_class = super_class.push(Value::from_string("Display"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("DEEPSEEK"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".deepseek_init".to_string(), Vec::new()));
    obj_class = obj_class.set("balance".to_string(), Value::ptr(".deepseek_balance".to_string(), Vec::new()));
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
