fn main() {
    println!("Hello, world!");
}
use std::error::Error;
use std::io::{Write, stderr};

fn print_error(err: &dyn Error) {
    writeln!(stderr(), "error {}", err);
    while let Some(source) = Error::source(err) {

        writeln!(stderr(), "caused by: {:?}", Error::source(err));
        err = Error::source(err);
    }
}