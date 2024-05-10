#![feature(iterator_try_collect)]

use std::io::Write;

use tempfile::NamedTempFile;

use crate::interpreter::{execute_file, reset_poison};

#[path = "../src/interpreter/mod.rs"]
mod interpreter;

mod builtin;
mod expression;
mod literal_expression;
mod oop;
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
