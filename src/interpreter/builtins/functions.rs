// FIXME: all of this mess

use std::{collections::HashMap, io, io::Write};

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    value::fru_value::{FruValue, TFnBuiltin},
    value::function::{AnyFunction, BuiltinFunction, EvaluatedArgumentList},
};

pub fn builtin_functions() -> HashMap<Identifier, FruValue> {
    HashMap::from(
        [
            (
                "print",
                b_print as TFnBuiltin,
            ),
            (
                "input",
                b_input as TFnBuiltin,
            ),
            (
                "assert_eq",
                b_assert_eq as TFnBuiltin,
            ),
        ].map(|(ident, function)|
            (
                Identifier::new(ident),
                FruValue::Function(AnyFunction::BuiltinFunction(BuiltinFunction::new(function)))
            )
        )
    )
}

fn b_print(args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
    for arg in args.args {
        print!("{:?} ", arg.1);
    }
    println!();

    Ok(FruValue::Nah)
}

fn b_input(args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
    if args.args.len() == 1 {
        print!("{:?}", args.args[0].1);
        io::stdout().flush().unwrap();
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Ok(FruValue::String(input.trim().to_string()))
}

fn b_assert_eq(args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
    if args.args[0].1 == args.args[1].1 {
        Ok(FruValue::Bool(true))
    } else {
        FruError::new_res(format!("assertion failed: {:?} != {:?}", args.args[0].1, args.args[1].1))
    }
}
