use std::{fmt::Debug, rc::Rc};

use crate::common::{returned_unit, FruError, FruStatement, FruValue, Identifier, Thing};

pub type TOpBuiltin = fn(FruValue, FruValue) -> Result<FruValue, FruError>;

#[derive(Clone)]
pub enum Operator {
    Operator {
        left_ident: Identifier,
        right_ident: Identifier,
        body: Rc<FruStatement>,
        scope: Thing,
    },
    Builtin(TOpBuiltin),
}

impl Operator {
    pub fn operate(&self, left_val: FruValue, right_val: FruValue) -> Result<FruValue, FruError> {
        match self {
            Operator::Operator {
                left_ident,
                right_ident,
                body,
                scope,
            } => {
                let new_scope = scope.derive_new();

                new_scope.let_prop(*left_ident, left_val)?;
                new_scope.let_prop(*right_ident, right_val)?;

                returned_unit(body.execute(new_scope))
            }

            Operator::Builtin(op) => op(left_val, right_val),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Builtin(_) => write!(f, "BuiltinOperator"),
            v => v.fmt(f),
        }
    }
}

// unsafe impl Send for AnyOperator {}
//
// unsafe impl Sync for AnyOperator {}
