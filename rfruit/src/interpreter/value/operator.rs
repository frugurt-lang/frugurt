use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::Control,
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

pub fn calculate_precedence(ident: &str) -> i32 {
    match ident {
        "=" => panic!("assignment is not an operator"),
        "||" => 6,
        "&&" => 5,
        "<" | "<=" | ">" | ">=" | "==" | "!=" => 4,
        "+" | "-" => 3,
        "*" | "/" | "%" => 2,
        "**" => 1,
        _ => -1,
    }
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

                let res = body.execute(new_scope);

                match res {
                    Control::Nah => Ok(FruValue::Nah),
                    Control::Return(v) => Ok(v),
                    Control::Error(e) => Err(e),
                    other => FruError::new_val(format!("unexpected signal {:?}", other)),
                }
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
