use std::fmt::Debug;

use crate::common::{derive_nat, INativeObject, IdOfObject};
use crate::value::fru_value::type_id;

pub struct BuiltinString {
    pub value: String,
    uid: IdOfObject,
}

impl BuiltinString {
    pub fn new(value: String) -> BuiltinString {
        BuiltinString {
            value,
            uid: IdOfObject::new(),
        }
    }
}

#[derive_nat(as_any, get_uid)]
impl INativeObject for BuiltinString {
    fn get_type_uid(&self) -> IdOfObject {
        *type_id::STRING_TYPE_ID
    }
}

impl Debug for BuiltinString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
