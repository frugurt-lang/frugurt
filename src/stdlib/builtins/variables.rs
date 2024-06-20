use std::collections::HashMap;

use macros::static_ident;

use crate::{
    interpreter::{identifier::Identifier, value::fru_value::FruValue},
    stdlib::builtins::{
        b_bool::BTypeBool, b_function::BTypeFunction, b_nah::BTypeNah, b_number::BTypeNumber,
        b_string::BTypeString, b_type::BTypeType, functions::builtin_functions,
    },
};

pub fn builtin_variables() -> HashMap<Identifier, FruValue> {
    let mut res = HashMap::new();

    res.extend(builtin_functions());

    // types

    res.extend([
        (static_ident!("Nah"), BTypeNah::get_value()),
        (static_ident!("Number"), BTypeNumber::get_value()),
        (static_ident!("Bool"), BTypeBool::get_value()),
        (static_ident!("String"), BTypeString::get_value()),
        (static_ident!("Function"), BTypeFunction::get_value()),
        (static_ident!("Type"), BTypeType::get_value()),
    ]);

    res
}
