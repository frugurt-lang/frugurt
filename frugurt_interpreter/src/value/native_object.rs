use std::{any::Any, fmt::Debug, rc::Rc};

use crate::common::{
    fru_err_res, EvaluatedArgumentList, FruError, FruValue, IdOfObject, Identifier,
};

pub trait INativeObject: Debug {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn get_uid(&self) -> IdOfObject;

    fn get_type_uid(&self) -> IdOfObject;

    fn call(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        fru_err_res!("cannot call `{:?}`", self)
    }

    fn index(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        fru_err_res!("cannot index `{:?}`", self)
    }

    fn get_prop(self: Rc<Self>, _ident: Identifier) -> Result<FruValue, FruError> {
        fru_err_res!("cannot get prop of `{:?}`", self)
    }

    fn set_prop(self: Rc<Self>, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        fru_err_res!("cannot set prop of `{:?}`", self)
    }

    fn let_prop(self: Rc<Self>, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        fru_err_res!("cannot let prop of `{:?}`", self)
    }
}

#[derive(Clone)]
pub struct NativeObject {
    internal: Rc<dyn INativeObject>,
}

impl NativeObject {
    pub fn new_value<T: INativeObject + 'static>(o: T) -> FruValue {
        FruValue::Native(Self {
            internal: Rc::new(o),
        })
    }

    pub fn new_value_rc<T: INativeObject + 'static>(o: Rc<T>) -> FruValue {
        FruValue::Native(Self { internal: o })
    }

    pub fn get_uid(&self) -> IdOfObject {
        self.internal.get_uid()
    }

    pub fn get_type_uid(&self) -> IdOfObject {
        self.internal.get_type_uid()
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.clone().call(args)
    }

    pub fn index(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.clone().index(args)
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.internal.clone().get_prop(ident)
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.internal.clone().set_prop(ident, value)
    }
}

pub fn cast_object<T: INativeObject + 'static>(o: &FruValue) -> Option<Rc<T>> {
    if let FruValue::Native(o) = o {
        o.internal.clone().as_any().downcast().ok()
    } else {
        None
    }
}

impl Debug for NativeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&self.internal, f)
    }
}

impl PartialEq for NativeObject {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.internal, &other.internal)
    }
}
