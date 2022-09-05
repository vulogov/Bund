
use std::collections;

const NONE: u16     = 0;
const BOOL: u16     = 1;
const INTEGER: u16  = 2;
const FLOAT: u16    = 3;
const STRING: u16   = 4;
const CALL: u16     = 5;
const PTR: u16      = 6;




#[derive(Clone)]
enum Val {
    Null,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(String),
}

pub struct Value {
    dt:         u16,
    pub q:      f32,
    data:       Val,
    tags:       collections::HashSet<String>,
}

impl Value {
    pub fn new() -> Self {
        Self {
            dt:   NONE,
            q:    0.0,
            data: Val::Null,
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_string(s: &String) -> Self {
        Self {
            dt:   STRING,
            q:    100.0,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_call(s: &String) -> Self {
        Self {
            dt:   CALL,
            q:    100.0,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_ptr(s: &String) -> Self {
        Self {
            dt:   PTR,
            q:    100.0,
            data: Val::String(s.to_string()),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_bool(v: &bool) -> Self {
        Self {
            dt:   BOOL,
            q:    100.0,
            data: Val::Bool(*v),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_int(v: &i64) -> Self {
        Self {
            dt:   INTEGER,
            q:    100.0,
            data: Val::I64(*v),
            tags: collections::HashSet::new(),
        }
    }
    pub fn from_float(v: &f64) -> Self {
        Self {
            dt:   FLOAT,
            q:    100.0,
            data: Val::F64(*v),
            tags: collections::HashSet::new(),
        }
    }
}

impl Value {
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
