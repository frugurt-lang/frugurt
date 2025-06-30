use std::rc::Rc;

use crate::common::{
    Control, FruExpression, FruValue, Identifier, Operator, OperatorIdentifier, Thing,
};

#[derive(Debug, Clone)]
pub enum FruStatement {
    SourceCode {
        body: Vec<FruStatement>,
    },
    Block {
        body: Vec<FruStatement>,
    },
    ObjectAlter {
        what: Box<FruExpression>,
        body: Vec<FruStatement>,
    },
    Expression {
        value: Box<FruExpression>,
    },
    Let {
        ident: Identifier,
        value: Box<FruExpression>,
    },
    If {
        condition: Box<FruExpression>,
        then_body: Box<FruStatement>,
        else_body: Option<Box<FruStatement>>,
    },
    While {
        condition: Box<FruExpression>,
        body: Box<FruStatement>,
    },
    Return {
        value: Option<Box<FruExpression>>,
    },
    Break,
    Continue,
    Operator {
        ident: Identifier,
        commutative: bool,
        left_ident: Identifier,
        left_type_ident: Identifier,
        right_ident: Identifier,
        right_type_ident: Identifier,
        body: Rc<FruStatement>,
    },
}

impl FruStatement {
    pub fn execute(&self, scope: Thing) -> Result<(), Control> {
        match self {
            FruStatement::SourceCode { body } => {
                for statement in body {
                    statement.execute(scope.clone())?;
                }
            }

            FruStatement::Block { body } => {
                let new_scope = scope.derive_new();

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }
            }

            FruStatement::ObjectAlter { what, body } => {
                let new_scope = match what.evaluate(scope)? {
                    FruValue::Thing(new_scope) => new_scope,

                    unexpected => {
                        return Control::new_err(format!("cannot alter `{:?}`", unexpected));
                    }
                };

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }
            }

            FruStatement::Expression { value } => {
                value.evaluate(scope.clone())?;
            }

            FruStatement::Let { ident, value } => {
                let v = value.evaluate(scope.clone())?;

                scope.let_prop(*ident, v)?;
            }

            FruStatement::If {
                condition,
                then_body,
                else_body,
            } => match condition.evaluate(scope.clone())? {
                FruValue::Bool(true) => then_body.execute(scope.clone())?,

                FruValue::Bool(false) => {
                    if let Some(else_body) = else_body {
                        else_body.execute(scope.clone())?
                    }
                }

                unexpected => {
                    return Control::new_err(format!(
                        "expected bool in condition, got `{:?}`",
                        unexpected
                    ));
                }
            },

            FruStatement::While { condition, body } => {
                while {
                    match condition.evaluate(scope.clone())? {
                        FruValue::Bool(b) => b,
                        unexpected => {
                            return Control::new_err(format!(
                                "expected `Bool` in condition, got `{:?}`",
                                unexpected
                            ));
                        }
                    }
                } {
                    if let Err(signal) = body.execute(scope.clone()) {
                        match signal {
                            Control::Continue => continue,
                            Control::Break => break,
                            Control::Return(v) => return Err(Control::Return(v)),
                            Control::Error(err) => return Err(Control::Error(err)),
                        }
                    }
                }
            }

            FruStatement::Return { value } => {
                return Err(Control::Return(match value {
                    Some(x) => x.evaluate(scope)?,
                    None => FruValue::Nah,
                }));
            }

            FruStatement::Break => return Err(Control::Break),
            FruStatement::Continue => return Err(Control::Continue),

            FruStatement::Operator {
                ident,
                commutative,
                left_ident,
                left_type_ident,
                right_ident,
                right_type_ident,
                body,
            } => {
                let left_type = scope.get_prop(*left_type_ident)?;
                let right_type = scope.get_prop(*right_type_ident)?;

                left_type.set_operator(
                    OperatorIdentifier::new(*ident, right_type.get_uid()),
                    Operator::Operator {
                        left_ident: *left_ident,
                        right_ident: *right_ident,
                        body: body.clone(),
                        scope: scope.clone(),
                    },
                )?;

                if *commutative {
                    right_type.set_operator(
                        OperatorIdentifier::new(*ident, left_type.get_uid()),
                        Operator::Operator {
                            left_ident: *right_ident,
                            right_ident: *left_ident,
                            body: body.clone(),
                            scope: scope.clone(),
                        },
                    )?;
                }
            }
        }

        Ok(())
    }
}
