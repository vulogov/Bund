use std::collections;
use crate::twostack::error::{BundError};

pub const NONE: u16     = 0;
pub const BOOL: u16     = 1;
pub const INTEGER: u16  = 2;
pub const FLOAT: u16    = 3;
pub const STRING: u16   = 4;
pub const LITERAL: u16  = 5;
pub const CALL: u16     = 6;
pub const PTR: u16      = 7;
pub const BIN: u16      = 8;
pub const ERROR: u16    = 98;
pub const TOKEN: u16    = 99;



#[derive(Clone)]
pub enum Val {
    Null,
    Token,
    Error(BundError),
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
    Binary(Vec<u8>),
}

#[derive(Clone)]
pub struct Value {
    dt:         u16,
    pub q:      f64,
    pub data:   Val,
    pub prefix: String,
    pub suffix: String,
    is_ready:   bool,
    pub is_attr:    bool,
    pub has_attr:   bool,
    tags:       collections::HashSet<String>,
}

impl Value {
    pub fn new() -> Self {
        Self {
            dt:   NONE,
            q:    0.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
            data: Val::Null,
            tags: collections::HashSet::new(),
        }
    }
    pub fn token() -> Self {
        Self {
            dt:   TOKEN,
            q:    0.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
            data: Val::Token,
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_string(s: &String) -> Self {
        Self {
            dt:   STRING,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
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
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
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
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
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
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
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
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
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
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
            data: Val::F64(*v),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_error(c: &String, m: &String) -> Self {
        Self {
            dt:   ERROR,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
            data: Val::Error(BundError::new(c.to_string(), m.to_string())),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_binary(d: &Vec<u8>) -> Self {
        Self {
            dt:   ERROR,
            q:    100.0,
            prefix: "".to_string(),
            suffix: "".to_string(),
            is_ready:  false,
            is_attr:   true,
            has_attr:  false,
            data: Val::Binary(d.to_vec()),
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
    pub fn binary(&mut self, s: &Vec<u8>) -> &mut Value {
        self.dt   = BIN;
        self.data = Val::Binary(s.to_vec());
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
    pub fn as_binary(&self) -> Option<&Vec<u8>> {
        match &self.data {
            Val::Binary(v) => Some(&v),
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
