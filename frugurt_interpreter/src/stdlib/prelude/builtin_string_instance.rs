use std::fmt::Debug;

use uid::Id;

use frugurt_macros::derive_nat;

use crate::common::*;
use crate::stdlib::common::BuiltinStringType;

pub struct BuiltinStringInstance {
    pub value: String,
    uid: Id<OfObject>,
}

impl BuiltinStringInstance {
    pub fn new(value: String) -> BuiltinStringInstance {
        BuiltinStringInstance {
            value,
            uid: Id::new(),
        }
    }
}

#[derive_nat(as_any, fru_clone)]
impl INativeObject for BuiltinStringInstance {
    fn get_uid(&self) -> Id<OfObject> {
        self.uid
    }

    fn get_type(&self) -> FruValue {
        BuiltinStringType::get_singleton()
    }
}

impl Debug for BuiltinStringInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
