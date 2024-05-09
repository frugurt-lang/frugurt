use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::returned_unit,
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    statement::FruStatement,
    value::fru_value::{FruValue, TOpBuiltin},
};

#[derive(Clone)]
pub enum AnyOperator {
    Operator {
        left_ident: Identifier,
        right_ident: Identifier,
        body: Rc<FruStatement>,
        scope: Rc<Scope>,
    },
    BuiltinOperator(TOpBuiltin),
}

impl AnyOperator {
    pub fn operate(&self, left_val: FruValue, right_val: FruValue) -> Result<FruValue, FruError> {
        match self {
            AnyOperator::Operator {
                left_ident,
                right_ident,
                body,
                scope,
            } => {
                let new_scope = Scope::new_with_parent(scope.clone());

                new_scope.let_variable(*left_ident, left_val)?;
                new_scope.let_variable(*right_ident, right_val)?;

                returned_unit(body.execute(new_scope))
            }

            AnyOperator::BuiltinOperator(op) => op(left_val, right_val),
        }
    }
}

impl Debug for AnyOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyOperator::BuiltinOperator(_) => write!(f, "BuiltinOperator"),
            v => v.fmt(f),
        }
    }
}
