use std::fmt::{Debug, Formatter};

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{
        fru_value::FruValue,
        native_object::{INativeObject, NativeObject},
    },
    static_native_value,
};

pub struct BuiltinFunctionType;

impl BuiltinFunctionType {
    pub fn get_value() -> FruValue {
        static_native_value!(BuiltinFunctionType)
    }
}

#[derive_nat(as_any, get_uid, get_type, fru_clone)]
impl INativeObject for BuiltinFunctionType {}

impl Debug for BuiltinFunctionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Function")
    }
}
