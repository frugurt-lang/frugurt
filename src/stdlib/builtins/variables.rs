use std::collections::HashMap;

use frugurt_macros::static_ident;

use crate::{
    interpreter::{identifier::Identifier, value::fru_value::FruValue},
    stdlib::builtins::{
        builtin_bool_type::BuiltinBoolType, builtin_function_type::BuiltinFunctionType,
        builtin_nah_type::BuiltinNahType, builtin_number_type::BuiltinNumberType,
        builtin_string_type::BuiltinStringType, builtin_type_type::BuiltinTypeType,
        functions::builtin_functions,
    },
};

pub fn builtin_variables() -> HashMap<Identifier, FruValue> {
    let mut res = HashMap::new();

    res.extend(builtin_functions());

    // types

    res.extend([
        (static_ident!("Nah"), BuiltinNahType::get_value()),
        (static_ident!("Number"), BuiltinNumberType::get_value()),
        (static_ident!("Bool"), BuiltinBoolType::get_value()),
        (static_ident!("String"), BuiltinStringType::get_value()),
        (static_ident!("Function"), BuiltinFunctionType::get_value()),
        (static_ident!("Type"), BuiltinTypeType::get_value()),
    ]);

    res
}
