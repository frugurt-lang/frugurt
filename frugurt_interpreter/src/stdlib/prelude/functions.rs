// FIXME: all of this mess

use std::{io, io::Write};

use crate::common::{
    fru_err_res, BuiltinFunction, EvaluatedArgumentList, FruError, FruValue, Identifier,
    NativeObject, Thing,
};
use crate::stdlib::common::BuiltinString;
use crate::value::builtin_function::TFnBuiltin;

pub fn builtin_functions(scope: Thing) -> Result<(), FruError> {
    for (ident, function) in [
        ("print", b_print as TFnBuiltin),
        ("input", b_input),
        ("assert_eq", b_assert_eq),
    ] {
        scope.let_prop(
            Identifier::new(ident),
            FruValue::BuiltinFunction(BuiltinFunction::new(function)),
        )?;
    }

    Ok(())
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
    Ok(NativeObject::new_value(BuiltinString::new(
        input.trim().to_string(),
    )))
}

fn b_assert_eq(args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
    if args.args[0].1 == args.args[1].1 {
        Ok(FruValue::Nah)
    } else {
        fru_err_res!(
            "assertion failed: {:?} != {:?}",
            args.args[0].1,
            args.args[1].1
        )
    }
}
