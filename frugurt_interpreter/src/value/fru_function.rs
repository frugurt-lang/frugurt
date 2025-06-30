use std::{fmt::Debug, rc::Rc};

use crate::common::{
    returned_unit, EvaluatedArgumentList, FormalParameters, FruError, FruStatement, FruValue, Thing,
};

#[derive(Clone)]
pub struct FruFunction {
    pub parameters: FormalParameters,
    pub body: Rc<FruStatement>,
    pub scope: Thing,
}

impl FruFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let new_scope = self.scope.derive_new();

        self.parameters.apply(args, new_scope.clone())?;

        returned_unit(self.body.execute(new_scope))
    }
}

impl Debug for FruFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function")
    }
}
