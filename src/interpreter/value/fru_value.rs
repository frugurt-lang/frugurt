use std::{cmp::PartialEq, fmt::Debug, rc::Rc};

use uid::Id;

use crate::{
    interpreter::{
        error::FruError,
        identifier::Identifier,
        value::{
            builtin_function::BuiltinFunction,
            curried::Curried,
            fru_function::FruFunction,
            fru_object::FruObject,
            fru_type::FruType,
            function_helpers::EvaluatedArgumentList,
            native_object::{NativeObject, OfObject},
        },
    },
    stdlib::builtins::{
        builtin_bool_type::BuiltinBoolType, builtin_function_type::BuiltinFunctionType,
        builtin_nah_type::BuiltinNahType, builtin_number_type::BuiltinNumberType,
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
    NativeObject(NativeObject),
}

impl FruValue {
    pub fn get_type(&self) -> FruValue {
        match self {
            FruValue::Nah => BuiltinNahType::get_value(),
            FruValue::Number(_) => BuiltinNumberType::get_value(),
            FruValue::Bool(_) => BuiltinBoolType::get_value(),
            FruValue::Function(_) => BuiltinFunctionType::get_value(),
            FruValue::BuiltinFunction(_) => BuiltinFunctionType::get_value(),
            FruValue::Curried(_) => BuiltinFunctionType::get_value(),
            FruValue::Type(_) => BuiltinNahType::get_value(),
            FruValue::Object(obj) => obj.get_type(),
            FruValue::NativeObject(obj) => obj.get_type(),
        }
    }

    pub fn get_uid(&self) -> Id<OfObject> {
        match self {
            FruValue::Type(obj) => obj.get_uid(),
            FruValue::Object(obj) => obj.get_uid(),
            FruValue::NativeObject(obj) => obj.get_uid(),

            _ => panic!(),
        }
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Function(fun) => fun.call(args),
            FruValue::BuiltinFunction(fun) => fun.call(args),
            FruValue::Curried(fun) => fun.call(args),
            FruValue::NativeObject(obj) => obj.call(args),
            _ => FruError::new_res(format!("`{:?}` is not invokable", self.get_type())),
        }
    }

    pub fn curry_call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Curried(curried) => Ok(curried.curry_call(args)),

            FruValue::Function(_) | FruValue::BuiltinFunction(_) | FruValue::NativeObject(_) => {
                Ok(Curried::new_value(self.clone(), args))
            }

            _ => FruError::new_res(format!("`{:?}` is not invokable", self.get_type())),
        }
    }

    pub fn instantiate(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(type_) => type_.instantiate(args),

            FruValue::NativeObject(obj) => obj.instantiate(args),

            _ => FruError::new_res(format!("`{:?}` is not instantiatable", self.get_type())),
        }
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(t) => t.get_prop(ident),

            FruValue::Object(obj) => obj.get_prop(ident),

            FruValue::NativeObject(obj) => obj.get_prop(ident),

            _ => FruError::new_res(format!("cannot access prop of `{:?}`", self.get_type())),
        }
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self {
            FruValue::Type(t) => t.set_prop(ident, value),

            FruValue::Object(obj) => obj.set_prop(ident, value),

            FruValue::NativeObject(obj) => obj.set_prop(ident, value),

            _ => FruError::new_res(format!("cannot set prop of `{:?}`", self.get_type())),
        }
    }

    pub fn fru_clone(&self) -> FruValue {
        match self {
            FruValue::Object(obj) => obj.fru_clone(),

            FruValue::NativeObject(obj) => obj.fru_clone(),

            _ => self.clone(),
        }
    }
}

impl PartialEq for FruValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FruValue::Nah, FruValue::Nah) => true,
            (FruValue::Number(left), FruValue::Number(right)) => left == right,
            (FruValue::Bool(left), FruValue::Bool(right)) => left == right,
            (FruValue::Type(left), FruValue::Type(right)) => left == right,
            (FruValue::Object(left), FruValue::Object(right)) => left == right,
            (FruValue::NativeObject(left), FruValue::NativeObject(right)) => left == right,
            _ => false,
        }
    }
}

impl Debug for FruValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FruValue::Nah => write!(f, "nah"),
            FruValue::Number(x) => write!(f, "{}", x),
            FruValue::Bool(x) => write!(f, "{}", x),
            FruValue::Function(x) => Debug::fmt(x, f),
            FruValue::BuiltinFunction(x) => Debug::fmt(x, f),
            FruValue::Curried(x) => Debug::fmt(x, f),
            FruValue::Type(x) => Debug::fmt(x, f),
            FruValue::Object(x) => Debug::fmt(x, f),
            FruValue::NativeObject(x) => Debug::fmt(x, f),
        }
    }
}

// interpreter is single threaded, so should be okay
unsafe impl Sync for FruValue {}

unsafe impl Send for FruValue {}
