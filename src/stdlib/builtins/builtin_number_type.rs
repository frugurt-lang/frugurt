use std::fmt::Debug;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{fru_value::FruValue, native_object::INativeObject},
    static_native_value,
};

pub struct BuiltinNumberType;

impl BuiltinNumberType {
    pub fn get_value() -> FruValue {
        static_native_value!(BuiltinNumberType)
    }
}

#[derive_nat(as_any, get_uid, get_type, get_set_op, fru_clone)]
impl INativeObject for BuiltinNumberType {}

impl Debug for BuiltinNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Number")
    }
}

// static_op1!();
// static OPERATORS: once_cell::sync::Lazy<
//     std::sync::Mutex<
//         std::collections::HashMap<
//             crate::interpreter::identifier::OperatorIdentifier,
//             crate::interpreter::value::operator::AnyOperator,
//         >,
//     >,
// > = once_cell::sync::Lazy::new(Default::default);
