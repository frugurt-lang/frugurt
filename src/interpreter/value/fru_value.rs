use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    error::FruError,
    identifier::id,
    identifier::Identifier,
    value::{
        fru_object::FruObject,
        fru_type::FruType,
        function::{AnyFunction, CurriedFunction, EvaluatedArgumentList, FruFunction},
        native::object::NativeObject,
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
    String(String),

    // function
    Function(AnyFunction),

    // oop
    Type(FruType),
    Object(FruObject),
    NativeObject(NativeObject),
}

impl FruValue {
    pub fn get_type_identifier(&self) -> Identifier {
        match self {
            FruValue::Nah => id::NAH,
            FruValue::Number(_) => id::NUMBER,
            FruValue::Bool(_) => id::BOOL,
            FruValue::String(_) => id::STRING,
            FruValue::Function(_) => id::FUNCTION,
            FruValue::Type(_) => id::TYPE,
            FruValue::Object(obj) => obj.get_type().get_ident(),
            FruValue::NativeObject(obj) => obj.get_type_identifier(),
        }
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Function(fun) => fun.call(args),
            FruValue::NativeObject(obj) => obj.call(args),
            _ => FruError::new_res(format!("`{}` is not invokable", self.get_type_identifier())),
        }
    }

    pub fn curry_call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Function(func) => {
                match func {
                    AnyFunction::CurriedFunction(func) => {
                        let mut new_args = func.saved_args.clone(); // TODO: obsidian Issue 1
                        new_args.args.extend(args.args);

                        Ok(FruValue::Function(AnyFunction::CurriedFunction(Rc::new(
                            CurriedFunction {
                                saved_args: new_args,
                                function: func.function.clone(),
                            },
                        ))))
                    }

                    normal => Ok(FruValue::Function(AnyFunction::CurriedFunction(Rc::new(
                        CurriedFunction {
                            saved_args: args,
                            function: Rc::new(normal.clone()),
                        },
                    )))),
                }
            }

            FruValue::NativeObject(obj) => obj.curry_call(args),

            _ => FruError::new_res(format!("`{}` is not invokable", self.get_type_identifier())),
        }
    }

    pub fn instantiate(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(type_) => type_.instantiate(args),

            FruValue::NativeObject(obj) => obj.instantiate(args),

            _ => FruError::new_res(format!(
                "`{}` is not instantiatable",
                self.get_type_identifier()
            )),
        }
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        match self {
            FruValue::Type(t) => t.get_prop(ident),

            FruValue::Object(obj) => obj.get_prop(ident),

            FruValue::NativeObject(obj) => obj.get_prop(ident),

            _ => FruError::new_res(format!(
                "cannot access prop of `{}`",
                self.get_type_identifier()
            )),
        }
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self {
            FruValue::Type(t) => t.set_prop(ident, value),

            FruValue::Object(obj) => obj.set_prop(ident, value),

            FruValue::NativeObject(obj) => obj.set_prop(ident, value),

            _ => FruError::new_res(format!(
                "cannot set prop of `{}`",
                self.get_type_identifier()
            )),
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

impl From<FruFunction> for FruValue {
    fn from(func: FruFunction) -> Self {
        FruValue::Function(AnyFunction::Function(Rc::new(func)))
    }
}

impl PartialEq for FruValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FruValue::Nah, FruValue::Nah) => true,
            (FruValue::Number(v1), FruValue::Number(v2)) => v1 == v2,
            (FruValue::Bool(v1), FruValue::Bool(v2)) => v1 == v2,
            (FruValue::String(v1), FruValue::String(v2)) => v1 == v2,
            (FruValue::Type(v1), FruValue::Type(v2)) => v1 == v2,
            (FruValue::Object(v1), FruValue::Object(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Debug for FruValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FruValue::Nah => write!(f, "nah"),
            FruValue::Number(v) => write!(f, "{}", v),
            FruValue::Bool(v) => write!(f, "{}", v),
            FruValue::String(v) => write!(f, "{}", v),
            FruValue::Function(fun) => write!(f, "{:?}", fun),
            FruValue::Type(type_) => write!(f, "{:?}", type_),
            FruValue::Object(obj) => write!(f, "{:?}", obj),
            FruValue::NativeObject(obj) => write!(f, "{}{{}}", obj.get_type_identifier()),
        }
    }
}
