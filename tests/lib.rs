extern crate interpreter;

use interpreter::interpreter::Interpreter;

mod builtin;
mod expression;
mod literal_expression;
mod oop;
mod scope_manipulation;
mod statement;

pub fn run(code: &str) {
    let interpreter = Interpreter::new();
    
    if let Err(err) = interpreter.execute_code(code.to_owned()) {
        panic!("{}", err)
    }
}
