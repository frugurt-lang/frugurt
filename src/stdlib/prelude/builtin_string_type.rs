use std::fmt::Debug;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{fru_value::FruValue, native_object::INativeObject},
    static_native_value,
};

pub struct BuiltinStringType;

impl BuiltinStringType {
    pub fn get_singleton() -> FruValue {
        static_native_value!(BuiltinStringType)
    }
}

#[derive_nat(as_any, get_uid, get_type, get_set_op, fru_clone)]
impl INativeObject for BuiltinStringType {}

impl Debug for BuiltinStringType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "String")
    }
}
