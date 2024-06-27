use std::fmt::Debug;

use crate::interpreter::{
    error::FruError,
    value::{
        fru_value::{FruValue, TFnBuiltin},
        function_helpers::EvaluatedArgumentList,
    },
};

#[derive(Clone)]
pub struct BuiltinFunction {
    function: TFnBuiltin,
}

impl BuiltinFunction {
    pub fn new(function: TFnBuiltin) -> Self {
        Self { function }
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        (self.function)(args)
    }
}

impl Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BuiltinFunction")
    }
}
