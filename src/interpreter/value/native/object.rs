use std::{any::Any, rc::Rc};

use crate::interpreter::{
    error::FruError,
    identifier::id,
    identifier::Identifier,
    value::{fru_value::FruValue, function::EvaluatedArgumentList},
};

pub trait INativeObject {
    fn as_any(&self) -> &dyn Any;

    fn get_type_identifier(&self) -> Identifier {
        id::NATIVE_OBJECT
    }

    fn call(&self, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        FruError::new_res(format!(
            "`{}` is not invokable ",
            self.get_type_identifier()
        ))
    }

    fn curry_call(&self, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        FruError::new_res(format!("`{}` is not invokable", self.get_type_identifier()))
    }

    fn instantiate(&self, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        FruError::new_res(format!(
            "`{}` is not instantiatable",
            self.get_type_identifier()
        ))
    }

    fn get_prop(&self, _ident: Identifier) -> Result<FruValue, FruError> {
        FruError::new_res(format!(
            "cannot access prop of `{}`",
            self.get_type_identifier()
        ))
    }

    fn set_prop(&self, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        FruError::new_res(format!(
            "cannot set prop of `{}`",
            self.get_type_identifier()
        ))
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject>;
}

#[derive(Clone)]
pub struct NativeObject {
    internal: Rc<dyn INativeObject>,
}

impl NativeObject {
    pub fn new(internal: Rc<dyn INativeObject>) -> Self {
        Self { internal }
    }

    pub fn get_type_identifier(&self) -> Identifier {
        self.internal.get_type_identifier()
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.call(args)
    }

    pub fn curry_call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.curry_call(args)
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

    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        self.internal.as_any().downcast_ref::<T>()
    }
}
