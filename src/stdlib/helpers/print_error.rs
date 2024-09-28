use easy_error::{Error};

pub fn print_error(err: Error) {
    println!("{}", err);
}
