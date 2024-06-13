use std::io::Write;

use tempfile::NamedTempFile;

use crate::interpreter::{identifier::reset_poison, runner::execute_file};

#[path = "../src/interpreter/mod.rs"]
mod interpreter;

#[path = "../src/stdlib/mod.rs"]
mod stdlib;

mod builtin;
mod expression;
mod literal_expression;
mod oop;
mod scope_manipulation;
mod statement;

pub fn run(code: &str) {
    let mut file = NamedTempFile::new().expect("failed to create temporary file");

    file.write_all(code.as_bytes()).expect("failed to write to temporary file");
    file.flush().unwrap();

    reset_poison();

    if let Err(err) = execute_file(file.path()) {
        panic!("{}", err)
    }
}
