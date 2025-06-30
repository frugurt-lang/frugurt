use crate::common::{
    fru_err_res, static_ident, BuiltinFunction, EvaluatedArgumentList, FruError, FruFunction,
    IdOfObject, Identifier, NativeObject, OperatorIdentifier, Thing,
};
use std::{cmp::PartialEq, fmt::Debug, rc::Rc};

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
    Thing(Thing),
    Native(NativeObject),
}
pub mod type_id {
    use crate::common::IdOfObject;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref NAH_TYPE_ID: IdOfObject = IdOfObject::new();
        pub static ref BOOL_TYPE_ID: IdOfObject = IdOfObject::new();
        pub static ref NUMBER_TYPE_ID: IdOfObject = IdOfObject::new();
        pub static ref FUNCTION_TYPE_ID: IdOfObject = IdOfObject::new();
        pub static ref STRING_TYPE_ID: IdOfObject = IdOfObject::new();
    }
}

impl FruValue {
    pub fn get_type_uid(&self) -> IdOfObject {
        match self {
            FruValue::Nah => *type_id::NAH_TYPE_ID,
            FruValue::Number(_) => *type_id::NUMBER_TYPE_ID,
            FruValue::Bool(_) => *type_id::BOOL_TYPE_ID,
            FruValue::Function(_) => *type_id::FUNCTION_TYPE_ID,
            FruValue::BuiltinFunction(_) => *type_id::FUNCTION_TYPE_ID,
            FruValue::Thing(obj) => {
                obj.get_prototype().map_or_else(|| obj.get_uid(), |x| x.get_uid())
            }
            FruValue::Native(obj) => obj.get_type_uid(),
        }
    }

    pub fn get_uid(&self) -> IdOfObject {
        match self {
            FruValue::Thing(obj) => obj.get_uid(),
            FruValue::Native(obj) => obj.get_uid(),

            _ => panic!(), // FIXME
        }
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            FruValue::Function(fun) => fun.call(args),
            FruValue::BuiltinFunction(fun) => fun.call(args),
            FruValue::Native(obj) => obj.call(args),
            _ => fru_err_res!("`{:?}` is not invokable", self),
        }
    }

    pub fn index(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            // FruValue::Thing(obj) => obj.index(args), // TODO: somehow
            FruValue::Native(obj) => obj.index(args),

            _ => fru_err_res!("`{:?}` is not indexable", self),
        }
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        match self {
            FruValue::Thing(obj) => obj.get_prop(ident),

            FruValue::Native(obj) => obj.get_prop(ident),

            _ => fru_err_res!("cannot access prop of `{:?}`", self),
        }
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self {
            FruValue::Thing(obj) => obj.set_prop(ident, value),

            FruValue::Native(obj) => obj.set_prop(ident, value),

            _ => fru_err_res!("cannot set prop of `{:?}`", self),
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
            (FruValue::Thing(left), FruValue::Thing(right)) => left == right,
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
            FruValue::Thing(x) => Debug::fmt(x, f),
            FruValue::Native(x) => Debug::fmt(x, f),
        }
    }
}

// interpreter is single threaded, so should be okay
// unsafe impl Sync for FruValue {}
//
// unsafe impl Send for FruValue {}
//
