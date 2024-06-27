use std::{
    fmt::{Debug, Formatter},
    rc::Rc,
};

use uid::Id;

use frugurt_macros::derive_nat;

use crate::{
    interpreter::{
        error::FruError,
        identifier::Identifier,
        scope::Scope,
        value::{
            fru_value::FruValue,
            native_object::{INativeObject, NativeObject, OfObject},
        },
    },
    stdlib::builtins::builtin_scope_type::BuiltinScopeType,
};

pub struct BuiltinScopeInstance {
    scope: Rc<Scope>,
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
        BuiltinScopeType::get_value()
    }

    fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.scope.get_variable(ident)
    }

    fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.scope.let_set_variable(ident, value);
        Ok(())
    }
}

impl Debug for BuiltinScopeInstance {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "scope")
    }
}

pub fn extract_scope_from_value(v: &FruValue) -> Option<Rc<Scope>> {
    if let FruValue::NativeObject(o) = v {
        o.downcast::<BuiltinScopeInstance>().map(|x| x.scope.clone())
    } else {
        None
    }
}
