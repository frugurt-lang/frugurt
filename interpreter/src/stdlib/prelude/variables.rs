use std::collections::HashMap;

use frugurt_macros::static_ident;

use crate::common::*;
use crate::stdlib::common::*;

pub fn builtin_variables() -> HashMap<Identifier, FruValue> {
    let mut res = HashMap::new();

    res.extend(builtin_functions());

    // types

    res.extend([
        (static_ident!("Nah"), BuiltinNahType::get_singleton()),
        (static_ident!("Number"), BuiltinNumberType::get_singleton()),
        (static_ident!("Bool"), BuiltinBoolType::get_singleton()),
        (static_ident!("String"), BuiltinStringType::get_singleton()),
        (
            static_ident!("Function"),
            BuiltinFunctionType::get_singleton(),
        ),
        (static_ident!("Type"), BuiltinTypeType::get_singleton()),
        (static_ident!("Vec"), BuiltinVecType::get_singleton()),
    ]);

    res
}
