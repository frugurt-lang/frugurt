use std::rc::Rc;

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    value::fru_value::FruValue,
    value::native::object::{INativeObject, NativeObject},
};

pub struct FruScope {
    scope: Rc<Scope>,
}

impl FruScope {
    pub fn new_value(scope: Rc<Scope>) -> FruValue {
        FruValue::NativeObject(NativeObject::new(Rc::new(Self { scope })))
    }
}

impl INativeObject for FruScope {
    fn get_type_identifier(&self) -> Identifier {
        Identifier::new("Scope")
    }

    fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.scope.get_variable(ident)
    }

    fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.scope
            .set_variable(ident, value.clone())
            .or_else(|_| self.scope.let_variable(ident, value))
    }
}
