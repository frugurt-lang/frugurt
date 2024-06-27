use std::{
    any::Any,
    fmt::{Debug, Formatter},
    rc::Rc,
};

use uid::Id;

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    value::{fru_value::FruValue, function_helpers::EvaluatedArgumentList},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OfObject;

pub trait INativeObject: Debug {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn get_uid(&self) -> Id<OfObject>;

    fn get_type(&self) -> FruValue;

    fn call(&self, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        FruError::new_res(format!("`{:?}` is not invokable ", self.get_type()))
    }

    fn instantiate(&self, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        FruError::new_res(format!("`{:?}` is not instantiatable", self.get_type()))
    }

    fn get_prop(&self, _ident: Identifier) -> Result<FruValue, FruError> {
        FruError::new_res(format!("cannot access prop of `{:?}`", self.get_type()))
    }

    fn set_prop(&self, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        FruError::new_res(format!("cannot set prop of `{:?}`", self.get_type()))
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject>;
}

#[derive(Clone)]
pub struct NativeObject {
    internal: Rc<dyn INativeObject>,
}

impl NativeObject {
    pub fn new_value<T: INativeObject + 'static>(o: T) -> FruValue {
        FruValue::NativeObject(Self {
            internal: Rc::new(o),
        })
    }

    pub fn get_type(&self) -> FruValue {
        self.internal.get_type()
    }

    pub fn get_uid(&self) -> Id<OfObject> {
        self.internal.get_uid()
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.call(args)
    }

    pub fn instantiate(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.instantiate(args)
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.internal.get_prop(ident)
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.internal.set_prop(ident, value)
    }

    pub fn fru_clone(&self) -> FruValue {
        FruValue::NativeObject(NativeObject {
            internal: self.internal.clone().fru_clone(),
        })
    }

    pub fn downcast<T: 'static>(&self) -> Option<Rc<T>> {
        self.internal.clone().as_any().downcast::<T>().ok()
    }
}

impl Debug for NativeObject {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.internal, f)
    }
}

impl PartialEq for NativeObject {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.internal, &other.internal)
    }
}
