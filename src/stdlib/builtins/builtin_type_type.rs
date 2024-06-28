use std::fmt::Debug;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{fru_value::FruValue, native_object::INativeObject},
    static_native_value,
};

pub struct BuiltinTypeType; // type of all types

impl BuiltinTypeType {
    pub fn get_value() -> FruValue {
        static_native_value!(BuiltinTypeType)
    }
}

#[derive_nat(as_any, get_uid, get_type, get_set_op, fru_clone)]
impl INativeObject for BuiltinTypeType {}

impl Debug for BuiltinTypeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type")
    }
}
