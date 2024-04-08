use std::rc::Rc;

use crate::interpreter::{error::FruError, identifier::Identifier, value::fru_value::FruValue};

pub trait INativeObject {
    fn get_type_identifier(&self) -> Identifier {
        Identifier::for_native_object()
    }

    fn call(&self, _args: Vec<FruValue>) -> Result<FruValue, FruError> {
        FruError::new_val(format!("cannot call {}", self.get_type_identifier()))
    }

    fn curry_call(&self, _args: Vec<FruValue>) -> Result<FruValue, FruError> {
        FruError::new_val(format!("cannot curry call {}", self.get_type_identifier()))
    }

    fn instantiate(&self, _args: Vec<FruValue>) -> Result<FruValue, FruError> {
        FruError::new_val(format!("cannot instantiate {}", self.get_type_identifier()))
    }

    fn get_field(&self, _ident: Identifier) -> Result<FruValue, FruError> {
        FruError::new_val(format!(
            "cannot get field of {}",
            self.get_type_identifier()
        ))
    }

    fn set_field(&self, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        FruError::new_unit(format!(
            "cannot set field of {}",
            self.get_type_identifier()
        ))
    }

    fn fru_clone(&self) -> FruValue {
        panic!();
    }
}

#[derive(Clone)]
pub struct NativeObject {
    pub internal: Rc<dyn INativeObject>,
}

impl NativeObject {
    pub fn get_type_identifier(&self) -> Identifier {
        self.internal.get_type_identifier()
    }

    pub fn call(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        self.internal.call(args)
    }

    pub fn curry_call(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        self.internal.curry_call(args)
    }

    pub fn instantiate(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        self.internal.instantiate(args)
    }

    pub fn get_field(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.internal.get_field(ident)
    }

    pub fn set_field(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.internal.set_field(ident, value)
    }

    pub fn fru_clone(&self) -> FruValue {
        self.internal.fru_clone()
    }
}
