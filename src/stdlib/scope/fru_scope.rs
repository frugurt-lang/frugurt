use std::{any::Any, rc::Rc};

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
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_identifier(&self) -> Identifier {
        Identifier::new("Scope")
    }

    fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.scope.get_variable(ident)
    }

    fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.scope.let_set_variable(ident, value);
        Ok(())
    }
}

pub fn extract_scope_from_value(v: &FruValue) -> Option<Rc<Scope>> {
    if let FruValue::NativeObject(o) = v {
        o.downcast::<FruScope>().map(|x| x.scope.clone())
    } else {
        None
    }
}
