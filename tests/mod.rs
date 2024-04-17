mod expression_tests;
mod function_tests;
mod oop_tests;
mod operators_tests;
mod scoping_tests;
mod variable_tests;

use crate::execute_file;

use tempfile::NamedTempFile;

use std::io::Write;

pub fn run(code: &str) {
    let mut file = NamedTempFile::new().expect("failed to create temporary file");

    file.write_all(code.as_bytes())
        .expect("failed to write to temporary file");
    file.flush().unwrap();

    execute_file(file.path()).unwrap();
}
