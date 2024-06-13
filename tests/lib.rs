use crate::interpreter::runner::execute_source_code;

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
    if let Err(err) = execute_source_code(code.to_owned()) {
        panic!("{}", err)
    }
}
