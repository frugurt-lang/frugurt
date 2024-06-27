use std::{fmt::Debug, rc::Rc};

use uid::Id;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::value::{
        fru_value::FruValue,
        native_object::{INativeObject, OfObject},
    },
    stdlib::builtins::builtin_string_type::BuiltinStringType,
};

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
        BuiltinStringType::get_value()
    }
}

impl TryFrom<&FruValue> for Rc<BuiltinStringInstance> {
    type Error = ();

    fn try_from(value: &FruValue) -> Result<Self, Self::Error> {
        let value = if let FruValue::NativeObject(x) = value {
            x
        } else {
            return Err(());
        };

        if let Some(s) = value.downcast::<BuiltinStringInstance>() {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl Debug for BuiltinStringInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
