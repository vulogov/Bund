use core::fmt::{self, Debug};
use crate::twostack::value::{Value, Val};

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.data {
            Val::Null => formatter.write_str("Null"),
            Val::Token => formatter.write_str("Token"),
            Val::Error(err) => write!(formatter, "Error({}:{})", err.class(), err.message()),
            Val::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Val::I64(number) => Debug::fmt(&number, formatter),
            Val::F64(number) => Debug::fmt(&number, formatter),
            Val::String(string) => write!(formatter, "String({:?})", string),
            Val::Binary(bin) => write!(formatter, "Binary({:?})", bin),
        }
    }
}
