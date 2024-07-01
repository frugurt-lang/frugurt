use std::{cmp::PartialEq, fmt::Debug, rc::Rc};

use uid::Id;

use frugurt_macros::static_ident;

use crate::{
    fru_err_res,
    interpreter::{
        error::FruError,
        identifier::{Identifier, OperatorIdentifier},
        value::{
            builtin_function::BuiltinFunction,
            curried::Curried,
            fru_function::FruFunction,
            fru_object::FruObject,
            fru_type::FruType,
            function_helpers::EvaluatedArgumentList,
            native_object::{NativeObject, OfObject},
            operator::AnyOperator,
        },
    },
    stdlib::prelude::{
        builtin_bool_type::BuiltinBoolType, builtin_function_type::BuiltinFunctionType,
        builtin_nah_type::BuiltinNahType, builtin_number_type::BuiltinNumberType,
        builtin_type_type::BuiltinTypeType,
    },
};

pub type TFnBuiltin = fn(EvaluatedArgumentList) -> Result<FruValue, FruError>;
pub type TOpBuiltin = fn(FruValue, FruValue) -> Result<FruValue, FruError>;

#[derive(Clone)]
pub enum FruValue {
    // primitives
    Nah,
    Number(f64),
    Bool(bool),

    // functions
    Function(Rc<FruFunction>),
    BuiltinFunction(BuiltinFunction),

    // specials
    Curried(Rc<Curried>),

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
            FruValue::Curried(_) => BuiltinFunctionType::get_singleton(), // FIXME
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
            FruValue::Curried(fun) => fun.call(args),
            FruValue::Native(obj) => obj.call(args),
            _ => fru_err_res!("`{:?}` is not invokable", self.get_type()),
        }
    }

    pub fn curry_call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Curried(curried) => Ok(curried.curry_call(args)),

            FruValue::Function(_) | FruValue::BuiltinFunction(_) | FruValue::Native(_) => {
                Ok(Curried::new_value(self.clone(), args))
            }

            _ => fru_err_res!("`{:?}` is not invokable", self.get_type()),
        }
    }

    pub fn instantiate(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(type_) => type_.instantiate(args),

            FruValue::Native(obj) => obj.instantiate(args),

            _ => fru_err_res!("`{:?}` is not instantiatable", self.get_type()),
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
            FruValue::Curried(x) => Debug::fmt(x, f),
            FruValue::Type(x) => Debug::fmt(x, f),
            FruValue::Object(x) => Debug::fmt(x, f),
            FruValue::Native(x) => Debug::fmt(x, f),
        }
    }
}

// interpreter is single threaded, so should be okay
unsafe impl Sync for FruValue {}

unsafe impl Send for FruValue {}
