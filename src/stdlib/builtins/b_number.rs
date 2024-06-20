use std::{
    any::Any,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use uid::Id;

use crate::{
    interpreter::value::{
        fru_value::FruValue,
        native_object::{INativeObject, NativeObject, OfObject},
    },
    static_fru_value, static_uid,
    stdlib::builtins::b_type::BTypeType,
};

pub struct BTypeNumber;

impl BTypeNumber {
    pub fn get_value() -> FruValue {
        static_fru_value!(BTypeNumber)
    }
}

impl INativeObject for BTypeNumber {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_uid(&self) -> Id<OfObject> {
        static_uid!()
    }

    fn get_type(&self) -> FruValue {
        NativeObject::new_value(BTypeType)
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject> {
        self
    }
}

impl Debug for BTypeNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number")
    }
}
