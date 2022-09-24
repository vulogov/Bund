use crate::twostack::value::{Value};

impl Value {
    pub fn convert_to_string(&self) -> String {
        format!("{}{}{}", &self.prefix, &self, &self.suffix).to_string()
    }
}
