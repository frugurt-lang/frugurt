use std::{any::Any, fmt::Debug, rc::Rc};

use uid::Id;

use crate::{
    interpreter::{
        error::FruError,
        identifier::Identifier,
        value::{
            fru_value::FruValue,
            function_helpers::EvaluatedArgumentList,
            native_object::{INativeObject, OfObject},
        },
    },
    stdlib::prelude::builtin_function_type::BuiltinFunctionType,
};

pub type SimpleMethodOfFn<T> = fn(&Rc<T>, EvaluatedArgumentList) -> Result<FruValue, FruError>;

pub struct SimpleMethodOf<T: INativeObject> {
    ident: Identifier,
    owner: Rc<T>,
    fun: SimpleMethodOfFn<T>,
    uid: Id<OfObject>,
}

impl<T: INativeObject> SimpleMethodOf<T> {
    pub fn new(ident: Identifier, owner: Rc<T>, fun: SimpleMethodOfFn<T>) -> Self {
        Self {
            ident,
            owner,
            fun,
            uid: Id::new(),
        }
    }
}

impl<T: INativeObject + 'static> INativeObject for SimpleMethodOf<T> {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

    fn get_uid(&self) -> Id<OfObject> {
        self.uid
    }

    fn get_type(&self) -> FruValue {
        BuiltinFunctionType::get_singleton()
    }

    fn call(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        (self.fun)(&self.owner, _args)
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject> {
        self
    }
}

impl<T: INativeObject> Debug for SimpleMethodOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}.{}", self.owner.get_type(), self.ident)
    }
}
