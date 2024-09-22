use std::{fmt::Debug, rc::Rc};

use uid::Id;

use frugurt_macros::derive_nat;

use crate::common::*;
use crate::stdlib::common::BuiltinScopeType;

pub struct BuiltinScopeInstance {
    pub scope: Rc<Scope>,
}

impl BuiltinScopeInstance {
    pub fn new_value(scope: Rc<Scope>) -> FruValue {
        NativeObject::new_value(Self { scope })
    }
}

#[derive_nat(as_any, fru_clone)]
impl INativeObject for BuiltinScopeInstance {
    fn get_uid(&self) -> Id<OfObject> {
        self.scope.uid
    }

    fn get_type(&self) -> FruValue {
        BuiltinScopeType::get_singleton()
    }

    fn get_prop(self: Rc<Self>, ident: Identifier) -> Result<FruValue, FruError> {
        self.scope.get_variable(ident)
    }

    fn set_prop(self: Rc<Self>, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.scope.let_set_variable(ident, value);
        Ok(())
    }
}

impl Debug for BuiltinScopeInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "scope")
    }
}
