use std::{cmp::PartialEq, fmt::Debug, rc::Rc};

use uid::Id;

use frugurt_macros::static_ident;

use crate::common::*;
use crate::stdlib::common::*;

pub type TFnBuiltin = fn(EvaluatedArgumentList) -> Result<FruValue, FruError>;
pub type TOpBuiltin = fn(FruValue, FruValue) -> Result<FruValue, FruError>;

#[derive(Clone)]
pub enum FruValue {
    // primitives
    Nah,
    Bool(bool),
    Number(f64),

    // functions
    Function(Rc<FruFunction>),
    BuiltinFunction(BuiltinFunction),

    // oop
    Type(FruType),
    Object(FruObject),
    Native(NativeObject),
}

impl FruValue {
    pub fn get_type(&self) -> FruValue {
        match self {
            FruValue::Nah => BuiltinNahType::get_singleton(),
            FruValue::Number(_) => BuiltinNumberType::get_singleton(),
            FruValue::Bool(_) => BuiltinBoolType::get_singleton(),
            FruValue::Function(_) => BuiltinFunctionType::get_singleton(),
            FruValue::BuiltinFunction(_) => BuiltinFunctionType::get_singleton(),
            FruValue::Type(_) => BuiltinTypeType::get_singleton(),
            FruValue::Object(obj) => obj.get_type(),
            FruValue::Native(obj) => obj.get_type(),
        }
    }

    pub fn get_uid(&self) -> Id<OfObject> {
        match self {
            FruValue::Type(obj) => obj.get_uid(),
            FruValue::Object(obj) => obj.get_uid(),
            FruValue::Native(obj) => obj.get_uid(),

            _ => panic!(), // FIXME
        }
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Function(fun) => fun.call(args),
            FruValue::BuiltinFunction(fun) => fun.call(args),
            FruValue::Native(obj) => obj.call(args),
            _ => fru_err_res!("`{:?}` is not invokable", self.get_type()),
        }
    }

    pub fn index(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(type_) => type_.index(args),

            FruValue::Native(obj) => obj.index(args),

            _ => fru_err_res!("`{:?}` is not indexable", self.get_type()),
        }
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(t) => t.get_prop(ident),

            FruValue::Object(obj) => obj.get_prop(ident),

            FruValue::Native(obj) => obj.get_prop(ident),

            _ => fru_err_res!("cannot access prop of `{:?}`", self.get_type()),
        }
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self {
            FruValue::Type(t) => t.set_prop(ident, value),

            FruValue::Object(obj) => obj.set_prop(ident, value),

            FruValue::Native(obj) => obj.set_prop(ident, value),

            _ => fru_err_res!("cannot set prop of `{:?}`", self.get_type()),
        }
    }

    pub fn get_operator(&self, ident: OperatorIdentifier) -> Option<AnyOperator> {
        match self {
            FruValue::Type(t) => t.get_operator(ident),

            FruValue::Native(obj) => obj.get_operator(ident),

            _ => panic!(),
        }
    }

    pub fn set_operator(
        &self,
        ident: OperatorIdentifier,
        value: AnyOperator,
    ) -> Result<(), FruError> {
        match self {
            FruValue::Type(t) => t.set_operator(ident, value),

            FruValue::Native(obj) => obj.set_operator(ident, value),

            _ => panic!(),
        }
    }

    pub fn fru_clone(&self) -> FruValue {
        match self {
            FruValue::Object(obj) => obj.fru_clone(),

            FruValue::Native(obj) => obj.fru_clone(),

            _ => self.clone(),
        }
    }
}

impl PartialEq for FruValue {
    // DELME
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FruValue::Nah, FruValue::Nah) => true,
            (FruValue::Number(left), FruValue::Number(right)) => left == right,
            (FruValue::Bool(left), FruValue::Bool(right)) => left == right,
            (FruValue::Type(left), FruValue::Type(right)) => left == right,
            (FruValue::Object(left), FruValue::Object(right)) => left == right,
            (FruValue::Native(left), FruValue::Native(right)) => {
                let op = left.get_type().get_operator(OperatorIdentifier::new(
                    static_ident!("=="),
                    right.get_type().get_uid(),
                ));
                if let Some(op) = op {
                    if let Ok(x) = op.operate(self.clone(), other.clone()) {
                        x == FruValue::Bool(true)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl Debug for FruValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FruValue::Nah => write!(f, "nah"),
            FruValue::Number(x) => Debug::fmt(x, f),
            FruValue::Bool(x) => Debug::fmt(x, f),
            FruValue::Function(x) => Debug::fmt(x, f),
            FruValue::BuiltinFunction(x) => Debug::fmt(x, f),
            FruValue::Type(x) => Debug::fmt(x, f),
            FruValue::Object(x) => Debug::fmt(x, f),
            FruValue::Native(x) => Debug::fmt(x, f),
        }
    }
}

// interpreter is single threaded, so should be okay
unsafe impl Sync for FruValue {}

unsafe impl Send for FruValue {}
