use std::{any::Any, fmt::Debug, fmt::Formatter, rc::Rc};

use uid::Id;

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
    static_fru_value, static_uid,
    stdlib::builtins::b_type::BTypeType,
};

pub struct BTypeScope;

pub struct BScope {
    scope: Rc<Scope>,
}

impl BScope {
    pub fn new_value(scope: Rc<Scope>) -> FruValue {
        NativeObject::new_value(Self { scope })
    }
}

impl INativeObject for BScope {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_uid(&self) -> Id<OfObject> {
        self.scope.uid
    }

    fn get_type(&self) -> FruValue {
        BTypeScope::get_value()
    }

    fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.scope.get_variable(ident)
    }

    fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.scope.let_set_variable(ident, value);
        Ok(())
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject> {
        self
    }
}

impl Debug for BScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope")
    }
}

pub fn extract_scope_from_value(v: &FruValue) -> Option<Rc<Scope>> {
    if let FruValue::NativeObject(o) = v {
        o.downcast::<BScope>().map(|x| x.scope.clone())
    } else {
        None
    }
}

impl BTypeScope {
    pub fn get_value() -> FruValue {
        static_fru_value!(BTypeScope)
    }
}

impl INativeObject for BTypeScope {
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

impl Debug for BTypeScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scope")
    }
}
