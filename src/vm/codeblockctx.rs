use std::error::Error;

pub struct CodeBlockCtx {
    code:       String,
    in_block:   bool,
}

impl CodeBlockCtx {
    pub fn new() -> Self {
        Self {
            code:           "".to_string(),
            in_block:       false,
        }
    }
}

impl CodeBlockCtx {
    pub fn code(&self) -> Result<&String, Box<dyn Error>> {
        Ok(&self.code)
    }
    pub fn is_in_codeblock(&self) -> bool {
        self.in_block
    }
    pub fn in_codeblock(&mut self) {
        self.in_block = true;
    }
    pub fn outof_codeblock(&mut self) {
        self.in_block = false;
        self.code = "".to_string()
    }
    pub fn set_code(&mut self, c: &String) {
        if self.is_in_codeblock() {
            self.code = self.code.to_owned() + c
        }
    }
}
