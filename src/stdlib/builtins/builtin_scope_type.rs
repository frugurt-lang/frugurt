use std::fmt::Debug;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{fru_value::FruValue, native_object::INativeObject},
    static_native_value,
};

pub struct BuiltinScopeType;

impl BuiltinScopeType {
    pub fn get_value() -> FruValue {
        static_native_value!(BuiltinScopeType)
    }
}

#[derive_nat(as_any, get_uid, get_type, get_set_op, fru_clone)]
impl INativeObject for BuiltinScopeType {}

impl Debug for BuiltinScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Scope")
    }
}
