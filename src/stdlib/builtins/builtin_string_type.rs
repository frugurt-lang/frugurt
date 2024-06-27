use std::fmt::{Debug, Formatter};

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{
        fru_value::FruValue,
        native_object::{INativeObject, NativeObject},
    },
    static_native_value,
};

pub struct BuiltinStringType;

impl BuiltinStringType {
    pub fn get_value() -> FruValue {
        static_native_value!(BuiltinStringType)
    }
}

#[derive_nat(as_any, get_uid, get_type, fru_clone)]
impl INativeObject for BuiltinStringType {}

impl Debug for BuiltinStringType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "String")
    }
}
