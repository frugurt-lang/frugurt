use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::returned_unit,
    error::FruError,
    scope::Scope,
    statement::FruStatement,
    value::{
        fru_value::FruValue,
        function_helpers::{EvaluatedArgumentList, FormalParameters},
    },
};

#[derive(Clone)]
pub struct FruFunction {
    pub parameters: FormalParameters,
    pub body: Rc<FruStatement>,
    pub scope: Rc<Scope>,
}

impl FruFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let new_scope = Scope::new_with_parent(self.scope.clone());

        self.parameters.apply(args, new_scope.clone())?;

        returned_unit(self.body.execute(new_scope))
    }
}

impl Debug for FruFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function")
    }
}
