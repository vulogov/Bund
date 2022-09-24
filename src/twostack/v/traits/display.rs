use core::fmt::{self, Display};
use crate::twostack::value::{Value, Val};

impl Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.data {
            Val::Null => formatter.write_str("Null"),
            Val::Token => formatter.write_str("Token"),
            Val::Error(err) => write!(formatter, "Error({}:{})", err.class(), err.message()),
            Val::Bool(boolean) => write!(formatter, "{}", boolean),
            Val::I64(number) => Display::fmt(&number, formatter),
            Val::F64(number) => Display::fmt(&number, formatter),
            Val::String(string) => write!(formatter, "{}", string),
            Val::Binary(bin) => write!(formatter, "{:?}", bin),
        }
    }
}
