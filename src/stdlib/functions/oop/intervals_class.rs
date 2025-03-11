extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use iset::IntervalMap;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub struct SampleInterval {
    m:  IntervalMap<f64, Value>,
}

impl SampleInterval {
    fn new() -> Self {
        Self {
            m: IntervalMap::new(),
        }
    }
    pub fn interval(self: &mut SampleInterval, lower: f64, upper: f64, label: Value) {
        if self.m.contains(lower..upper) {
            return;
        }
        let _ = self.m.insert(lower..upper, label);
    }
    pub fn check(self: &mut SampleInterval, value: f64) -> Result<Value, Error> {
        for (_, v) in self.m.overlap(value) {
            return Ok(v.clone());
        }
        bail!("Interval key error: {}", &value);
    }
    pub fn len(self: &mut SampleInterval) -> i64 {
        self.m.len() as i64
    }
}

fn register_method_intervals_list_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Floats: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            let mut new_data = Value::list();
            for v in data_object {
                let v_list = match v.cast_list() {
                    Ok(v_list) => v_list,
                    Err(err) => bail!("Intervals: element of intervals is not a iterable: {}", err),
                };
                if v_list.len() != 2 {
                    bail!("Intervals: element of intervals is not suitable for begin..end");
                }
                let v_start_value = &v_list[0];
                let v_end_value = &v_list[1];
                let v_start = match v_start_value.conv(FLOAT) {
                    Ok(v_start) => match v_start.cast_float() {
                        Ok(v_start) => v_start,
                        Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_start_value, err),
                    },
                    Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_start_value, err),
                };
                let v_end = match v_end_value.conv(FLOAT) {
                    Ok(v_end) => match v_end.cast_float() {
                        Ok(v_end) => v_end,
                        Err(err) => bail!("Interval: error making end of interval: {}: {}", &v_start_value, err),
                    },
                    Err(err) => bail!("Interval: error making end of interval: {}: {}", &v_start_value, err),
                };
                let mut interval_list = Value::list();
                interval_list = interval_list.push(Value::from_float(v_start));
                interval_list = interval_list.push(Value::from_float(v_end));
                new_data = new_data.push(interval_list);
            }
            let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, new_data);
            vm.stack.push(new_value_object);
        }
        None => bail!("Intervals: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_method_intervals_push(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for method 'Intervals::push'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Intervals: NO DATA #1"),
    };
    let new_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Intervals: NO DATA #2"),
    };

    let v_list = match new_object.cast_list() {
        Ok(v_list) => v_list,
        Err(err) => bail!("Intervals: element of intervals is not a iterable: {}", err),
    };
    if v_list.len() != 2 {
        bail!("Intervals: element of intervals is not suitable for begin..end");
    }
    let v_start_value = &v_list[0];
    let v_end_value = &v_list[1];
    let v_start = match v_start_value.conv(FLOAT) {
        Ok(v_start) => match v_start.cast_float() {
            Ok(v_start) => v_start,
            Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_start_value, err),
        },
        Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_start_value, err),
    };
    let v_end = match v_end_value.conv(FLOAT) {
        Ok(v_end) => match v_end.cast_float() {
            Ok(v_end) => v_end,
            Err(err) => bail!("Interval: error making end of interval: {}: {}", &v_start_value, err),
        },
        Err(err) => bail!("Interval: error making end of interval: {}: {}", &v_start_value, err),
    };
    let mut interval_list = Value::list();
    interval_list = interval_list.push(Value::from_float(v_start));
    interval_list = interval_list.push(Value::from_float(v_end));
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(mut data_object) => {
            if data_object.type_of() == LIST {
                data_object = data_object.push(interval_list);
            } else {
                bail!("Intervals::push data object is not LIST");
            }
            let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, data_object);
            vm.stack.push(new_value_object);
        }
        None => bail!("Intervals: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_method_intervals_overlap(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for method 'Intervals::overlap'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Intervals: NO DATA #1"),
    };
    let check_value = match vm.stack.pull() {
        Some(value_object) => match value_object.conv(FLOAT) {
            Ok(value_object) => match value_object.cast_float() {
                Ok(value_object) => value_object,
                Err(err) => bail!("Intervals: error casting check value: {}", err),
            }
            Err(err) => bail!("Intervals: error converting check value: {}", err),
        },
        None => bail!("Intervals: NO DATA #2"),
    };


    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            let mut intervals = SampleInterval::new();
            for i in data_object {
                let single_interval = match i.cast_list() {
                    Ok(single_interval) => single_interval,
                    Err(err) => bail!("Intervals: element of stored intervals is not a iterable: {}", err),
                };
                if single_interval.len() != 2 {
                    bail!("Intervals: element of stored intervals is not suitable for begin..end");
                }
                let v_si_start_value = &single_interval[0];
                let v_si_end_value = &single_interval[1];
                let v_si_start = match v_si_start_value.conv(FLOAT) {
                    Ok(v_start) => match v_start.cast_float() {
                        Ok(v_start) => v_start,
                        Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_si_start_value, err),
                    },
                    Err(err) => bail!("Interval: error making start of interval: {}: {}", &v_si_start_value, err),
                };
                let v_si_end = match v_si_end_value.conv(FLOAT) {
                    Ok(v_end) => match v_end.cast_float() {
                        Ok(v_end) => v_end,
                        Err(err) => bail!("Interval: error making end of stored interval: {}: {}", &v_si_end_value, err),
                    },
                    Err(err) => bail!("Interval: error making end of stored interval: {}: {}", &v_si_end_value, err),
                };
                intervals.interval(v_si_start,v_si_end, Value::from_string(format!("{}..{}", v_si_start,v_si_end)));
            }
            match intervals.check(check_value) {
                Ok(int_key) => vm.stack.push(int_key),
                Err(err) => bail!("Intervals::overlap returned error: {}", err),
            };
        }
        None => bail!("Intervals: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_intervals(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".intervals_init".to_string(), register_method_intervals_list_init);
    let _ = vm.register_method(".intervals_push".to_string(), register_method_intervals_push);
    let _ = vm.register_method(".intervals_overlap".to_string(), register_method_intervals_overlap);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("List"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Intervals"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".intervals_init".to_string(), Vec::new()));
    obj_class = obj_class.set("push".to_string(), Value::ptr(".intervals_push".to_string(), Vec::new()));
    obj_class = obj_class.set("overlap".to_string(), Value::ptr(".intervals_overlap".to_string(), Vec::new()));
    vm.register_class("Intervals".to_string(), obj_class)
}

pub fn stdlib_object_intervals_value_empty(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::from_list(Vec::new()));
    vm.stack.push(Value::from_string("Intervals"));
    vm.apply(Value::call("object".to_string(), Vec::new()))
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    //
    // Register Value
    //
    match register_intervals(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Intervals base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    let _ = bc.vm.register_inline("Intervals".to_string(), stdlib_object_intervals_value_empty);
    drop(bc);
}
