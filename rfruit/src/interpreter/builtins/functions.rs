use std::{collections::HashMap, io, io::Write};

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    value::fru_value::FruValue,
    value::function::{AnyFunction, ArgCount, BuiltinFunction},
};

pub fn builtin_functions() -> HashMap<Identifier, FruValue> {
    HashMap::from([
        (
            Identifier::new("print"),
            FruValue::Function(AnyFunction::BuiltinFunction(BuiltinFunction {
                function: b_print,
                argument_count: ArgCount::Any,
            })),
        ),
        (
            Identifier::new("input"),
            FruValue::Function(AnyFunction::BuiltinFunction(BuiltinFunction {
                function: b_input,
                argument_count: ArgCount::AtMost(1),
            })),
        ),
        (
            Identifier::new("t_eq"),
            FruValue::Function(AnyFunction::BuiltinFunction(BuiltinFunction {
                function: b_t_eq,
                argument_count: ArgCount::Exact(2),
            })),
        ),
    ])
}

fn b_print(args: Vec<FruValue>) -> Result<FruValue, FruError> {
    for arg in args {
        print!("{:?} ", arg);
    }
    println!();

    Ok(FruValue::Nah)
}

fn b_input(args: Vec<FruValue>) -> Result<FruValue, FruError> {
    if args.len() == 1 {
        print!("{:?}", args[0]);
        io::stdout().flush().unwrap();
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Ok(FruValue::String(input.trim().to_string()))
}

fn b_t_eq(args: Vec<FruValue>) -> Result<FruValue, FruError> {
    if args[0] == args[1] {
        Ok(FruValue::Bool(true))
    } else {
        panic!("TEST FAILED: {:?} != {:?}", args[0], args[1]);
    }
}
