use std::{any::Any, fmt::Formatter, rc::Rc};

use uid::Id;

use crate::{
    interpreter::value::{
        fru_value::FruValue,
        native::object::{INativeObject, NativeObject, OfObject},
    },
    static_fru_value, static_uid,
    stdlib::builtins::b_type::BTypeType,
};

pub struct BTypeNah;

impl BTypeNah {
    pub fn get_value() -> FruValue {
        static_fru_value!(BTypeNah)
    }
}

impl INativeObject for BTypeNah {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_uid(&self) -> Id<OfObject> {
        static_uid!()
    }

    fn get_type(&self) -> FruValue {
        NativeObject::new_value(BTypeType)
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nah")
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject> {
        self
    }
}
