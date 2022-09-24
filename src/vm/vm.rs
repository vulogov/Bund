extern crate log;
use std::collections;
use crate::twostack;
use crate::twostack::value;
use crate::vm::bundfunction;
use crate::vm::traceback;
use crate::vm::codeblockctx;
use crate::vm::bundcontext;
use crate::vm::bundstate;
use crate::stdlib::bund::{init_stdlib};


pub struct VM {
    tb:         collections::VecDeque<traceback::Traceback>,
    v:          collections::VecDeque<value::Value>,
    a:          collections::VecDeque<collections::VecDeque<value::Value>>,
    ts:         twostack::TS,
    functions:  collections::HashMap<String,bundfunction::BundFunction>,
    cbctx:      codeblockctx::CodeBlockCtx,
    ctx:        bundcontext::BundContext,
pub state:      bundstate::BundState,
}

impl VM {
    pub fn new() -> Self {
        log::debug!("Creating BUND VM");
        Self {
            ts:             twostack::TS::new(),
            v:              collections::VecDeque::new(),
            a:              collections::VecDeque::new(),
            functions:      collections::HashMap::new(),
            tb:             collections::VecDeque::new(),
            cbctx:          codeblockctx::CodeBlockCtx::new(),
            ctx:            bundcontext::BundContext::new(),
            state:          bundstate::BundState::new(),
        }
    }
    pub fn init() -> Self {
        let mut b = VM::new();
        log::trace!("Initializing BUND standard library");
        init_stdlib(&mut b);
        log::trace!("BUND VM is ready");
        b
    }
}

impl VM {
    pub fn local(&mut self) -> &mut collections::VecDeque<value::Value> {
        self.ts.local()
    }
    pub fn take_stack(&mut self) -> collections::VecDeque<value::Value> {
        self.ts.take_stack()
    }
    pub fn to_attr(&mut self, s: collections::VecDeque<value::Value>) {
        self.a.push_back(s)
    }
    pub fn attr(&mut self) -> Option<collections::VecDeque<value::Value>> {
        if self.a.is_empty() {
            return None;
        }
        log::trace!("Taking attr from attribute cache");
        self.a.pop_back()
    }
    pub fn get(&mut self) -> Option<value::Value> {
        self.ts.get()
    }
    pub fn look(&mut self) -> Option<&value::Value> {
        self.ts.look()
    }
    pub fn set(&mut self, v: value::Value)  {
        self.ts.set(v)
    }
    pub fn set_by_ref(&mut self, v: &value::Value)  {
        self.ts.set_by_ref(v)
    }
    pub fn drop_function(&mut self, name: &String) -> Option<bundfunction::BundFunction> {
        self.functions.remove(name)
    }
    pub fn add_function(&mut self, n: &str, f: bundfunction::BundFunction) {
        log::debug!("Registering BUND function: {:?}", &n);
        self.drop_function(&n.to_string());
        self.functions.insert(n.to_string(), f);
    }
    pub fn function(&self, name: &String) -> Option<&bundfunction::BundFunction> {
        self.functions.get(name)
    }
    pub fn value(&mut self) -> Option<value::Value> {
        if self.v.is_empty() {
            return None;
        }
        log::trace!("Taking value from value cache");
        self.v.pop_back()
    }
    pub fn add_value(&mut self, v: value::Value) {
        log::trace!("Adding value from lexer: {:?}", &v);
        self.v.push_back(v)
    }
}

impl VM {
    pub fn new_stack(&mut self) -> &mut collections::VecDeque<value::Value> {
        self.ts.new_stack()
    }
    pub fn new_named_stack(&mut self, n: &String) -> &mut collections::VecDeque<value::Value> {
        self.ts.new_named_stack(n)
    }
    pub fn drop_stack(&mut self)  {
        self.ts.drop_stack();
    }
}

impl VM {
    pub fn set_error(&mut self, c: String, m: String) {
        self.ts.set(value::Value::from_error(&c, &m))
    }
    pub fn is_error(&mut self) -> bool {
        let v = self.look().unwrap();
        match v.type_of() {
            value::ERROR => {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn to_traceback(&mut self, s: String) {
        self.tb.push_back(traceback::Traceback::new(s))
    }
    pub fn dump_traceback(&self) {
        let mut c = 0;
        while c < self.tb.len() {
            c += 1;
            let t = self.tb.get(c).unwrap();
            println!("[{:?}] {}", t.elapsed(), t.rule());
        }
    }
}

impl VM {
    pub fn in_codeblock(&mut self)  {
        self.cbctx.in_codeblock()
    }
    pub fn outof_codeblock(&mut self)  {
        self.cbctx.outof_codeblock()
    }
    pub fn code(&mut self, n: String) -> Option<&String> {
        self.ctx.get(n)
    }
    pub fn add_code(&mut self, c: &String)  {
        self.cbctx.set_code(c)
    }
    pub fn set_code_in_ctx(&mut self, n: String) {
        if self.cbctx.is_in_codeblock() {
            let code = &mut self.cbctx.code().unwrap();
            let _ = &mut self.ctx.set(n, code.to_string());
            let _ = &mut self.cbctx.outof_codeblock();
        }
    }
}
