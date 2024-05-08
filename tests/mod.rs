mod builtin;
mod expression;
mod literal_expression;
mod oop;
mod statement;

use std::io::Write;

use tempfile::NamedTempFile;

use crate::interpreter::{
    runner::execute_file,
    identifier,
};


pub fn run(code: &str) {
    let mut file = NamedTempFile::new().expect("failed to create temporary file");

    file.write_all(code.as_bytes())
        .expect("failed to write to temporary file");
    file.flush().unwrap();

    identifier::reset_poison();

    execute_file(file.path()).unwrap();
}
