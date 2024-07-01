use std::{any::Any, fmt::Debug, rc::Rc};

use uid::Id;

use crate::{
    fru_err_res,
    interpreter::{
        error::FruError,
        identifier::{Identifier, OperatorIdentifier},
        value::{
            fru_value::FruValue, function_helpers::EvaluatedArgumentList, operator::AnyOperator,
        },
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OfObject;

pub trait INativeObject: Debug {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn get_uid(&self) -> Id<OfObject>;

    fn get_type(&self) -> FruValue;

    fn call(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        fru_err_res!("`{:?}` is not invokable ", self.get_type())
    }

    fn instantiate(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        fru_err_res!("`{:?}` is not instantiatable", self.get_type())
    }

    fn get_prop(self: Rc<Self>, _ident: Identifier) -> Result<FruValue, FruError> {
        fru_err_res!("cannot access prop of `{:?}`", self.get_type())
    }

    fn set_prop(self: Rc<Self>, _ident: Identifier, _value: FruValue) -> Result<(), FruError> {
        fru_err_res!("cannot set prop of `{:?}`", self.get_type())
    }

    fn get_operator(self: Rc<Self>, _ident: OperatorIdentifier) -> Option<AnyOperator> {
        unimplemented!();
    }

    fn set_operator(
        self: Rc<Self>,
        _ident: OperatorIdentifier,
        _value: AnyOperator,
    ) -> Result<(), FruError> {
        unimplemented!();
    }

    fn fru_clone(self: Rc<Self>) -> Rc<dyn INativeObject>;
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

    pub fn get_type(&self) -> FruValue {
        self.internal.get_type()
    }

    pub fn get_uid(&self) -> Id<OfObject> {
        self.internal.get_uid()
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.clone().call(args)
    }

    pub fn instantiate(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        self.internal.clone().instantiate(args)
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        self.internal.clone().get_prop(ident)
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        self.internal.clone().set_prop(ident, value)
    }

    pub fn get_operator(&self, ident: OperatorIdentifier) -> Option<AnyOperator> {
        self.internal.clone().get_operator(ident)
    }

    pub fn set_operator(
        &self,
        ident: OperatorIdentifier,
        value: AnyOperator,
    ) -> Result<(), FruError> {
        self.internal.clone().set_operator(ident, value)
    }

    pub fn fru_clone(&self) -> FruValue {
        FruValue::Native(NativeObject {
            internal: self.internal.clone().fru_clone(),
        })
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