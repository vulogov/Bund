extern crate log;
use crate::vm::vm::{VM};

impl VM {
    pub fn begin_codeblock(&mut self, n: &String) -> bool {
        log::trace!(">CB: {:?} {:?} {:?}", &self.state.requestor_name, &n, self.state.codeblock_requested);
        if self.state.requestor_name.is_empty() && self.state.codeblock_requested  {
            log::debug!("Beginning codeblock for {}", &n);
            self.state.setrequestor(n);
            self.in_codeblock();
            self.add_code(format!("/* {} */ ", &n));
            return true;
        }
        return false;
    }
    pub fn end_codeblock(&mut self, n: &String) -> bool {
        log::trace!("<CB: {:?} {:?} {:?}", &self.state.requestor_name, &n, self.state.codeblock_requested);
        if &self.state.requestor_name == n && self.state.codeblock_requested  {
            log::debug!("Ending codeblock for {}", &n);
            self.set_code_in_ctx(n.to_string());
            self.outof_codeblock();
            self.state.requestor_name.clear();
            self.state.codeblock_requested = false;
            return true;
        }
        return false;
    }
}
