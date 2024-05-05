mod control_tests;
mod expression_tests;
mod function_tests;
mod if_expression_tests;
mod if_statement_tests;
mod let_set_tests;
mod oop_tests;
mod operators_tests;
mod return_statement_tests;
mod scoping_tests;
mod value_tests;
mod while_statement_tests;
mod while_tests;

use crate::execute_file;

use tempfile::NamedTempFile;

use std::io::Write;
use crate::interpreter::identifier;

pub fn run(code: &str) {
    let mut file = NamedTempFile::new().expect("failed to create temporary file");

    file.write_all(code.as_bytes())
        .expect("failed to write to temporary file");
    file.flush().unwrap();

    identifier::reset_poison();

    execute_file(file.path()).unwrap();
}
