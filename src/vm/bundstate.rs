extern crate log;

#[derive(Clone)]
pub struct BundState {
    pub codeblock_requested: bool,
    pub get_all_requested:   bool,
    pub defineptr_requested: bool,
    pub requestor_name:      String,
}

impl BundState {
    pub fn new() -> Self {
        Self {
            codeblock_requested:    false,
            get_all_requested:      false,
            defineptr_requested:    false,
            requestor_name:         String::new(),
        }
    }
    pub fn setrequestor(&mut self, n: &String) {
        if self.requestor_name.len() == 0 && (self.codeblock_requested || self.get_all_requested || self.defineptr_requested) {
            log::debug!("Set requestor name for BUND state {}", &n);
            self.requestor_name = n.to_string();
        }
    }
    pub fn clear(&mut self, n: &String) {
        if &self.requestor_name == n && (self.codeblock_requested || self.get_all_requested || self.defineptr_requested) {
            log::debug!("Clearing BUND state in {}", &n);
            self.requestor_name.clear();
            self.codeblock_requested = false;
            self.get_all_requested = false;
            self.defineptr_requested = false;
        }
    }
}
