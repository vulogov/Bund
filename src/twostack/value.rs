use core::fmt::{self, Debug};
use std::collections;

const NONE: u16     = 0;
const BOOL: u16     = 1;
const INTEGER: u16  = 2;
const FLOAT: u16    = 3;
const STRING: u16   = 4;
const LITERAL: u16  = 5;
const CALL: u16     = 6;
const PTR: u16      = 7;




#[derive(Clone)]
enum Val {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
}

#[derive(Clone)]
pub struct Value {
    dt:         u16,
    pub q:      f32,
    data:       Val,
    pub prefix:     String,
    pub suffix:     String,
    is_ready:   bool,
    tags:       collections::HashSet<String>,
}

impl Value {
    pub fn new() -> Self {
        Self {
            dt:   NONE,
            q:    0.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::Null,
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_string(s: &String) -> Self {
        Self {
            dt:   STRING,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_call(s: &String) -> Self {
        Self {
            dt:   CALL,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_ptr(s: &String) -> Self {
        Self {
            dt:   PTR,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_bool(v: &bool) -> Self {
        Self {
            dt:   BOOL,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::Bool(*v),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_int(v: &i64) -> Self {
        Self {
            dt:   INTEGER,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::I64(*v),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_float(v: &f64) -> Self {
        Self {
            dt:   FLOAT,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:   false,
            data: Val::F64(*v),
            tags: collections::HashSet::new(),
        }
    }
}

impl Value {
    pub fn string(&mut self, s: &String) -> &mut Value {
        self.dt   = STRING;
        self.data = Val::String(s.to_string());
        self
    }
    pub fn literal(&mut self, s: &String) -> &mut Value {
        self.dt   = LITERAL;
        self.data = Val::String(s.to_string());
        self
    }
    pub fn float(&mut self, v: f64) -> &mut Value {
        self.dt   = FLOAT;
        self.data = Val::F64(v);
        self
    }
    pub fn int(&mut self, v: i64) -> &mut Value {
        self.dt   = INTEGER;
        self.data = Val::I64(v);
        self
    }
    pub fn bool(&mut self, v: bool) -> &mut Value {
        self.dt   = BOOL;
        self.data = Val::Bool(v);
        self
    }
    pub fn call(&mut self, s: &String) -> &mut Value {
        self.dt   = CALL;
        self.data = Val::String(s.to_string());
        self
    }
    pub fn ptr(&mut self, s: &String) -> &mut Value {
        self.dt   = PTR;
        self.data = Val::String(s.to_string());
        self
    }
}

impl Value {
    pub fn to_ready(&mut self) -> &Value {
        self.is_ready = true;
        self
    }
    pub fn to_not_ready(&mut self) -> &Value {
        self.is_ready = false;
        self
    }
    pub fn is_ready(&self) -> bool {
        self.is_ready
    }
    pub fn type_of(&self) -> u16 {
        self.dt
    }
    pub fn to_ptr(&mut self) -> &Value {
        if &self.dt == &CALL {
            self.dt = PTR;
        }
        self
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self.data {
            Val::Bool(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self.data {
            Val::I64(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self.data {
            Val::F64(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&String> {
        match &self.data {
            Val::String(v) => Some(&v),
            _ => None,
        }
    }
    pub fn prefix(&mut self, p: &String) {
        self.prefix = p.to_string()
    }
    pub fn suffix(&mut self, s: &String) {
        self.suffix = s.to_string()
    }
    pub fn clear_tags(&mut self) -> &Value {
        let _ = &self.tags.clear();
        self
    }
    pub fn set_tag(&mut self, s: &String) -> &Value {
        let _ = &self.tags.insert(s.to_string());
        self
    }
    pub fn tags_of(&mut self) -> &mut collections::HashSet<String>  {
        &mut self.tags
    }
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.data {
            Val::Null => formatter.write_str("Null"),
            Val::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Val::I64(number) => Debug::fmt(&number, formatter),
            Val::F64(number) => Debug::fmt(&number, formatter),
            Val::String(string) => write!(formatter, "String({:?})", string),
        }
    }
}
