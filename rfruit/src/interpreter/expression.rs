use std::rc::Rc;

use crate::interpreter::{
    control::Control,
    error::FruError,
    identifier::{Identifier, OperatorIdentifier},
    scope::Scope,
    statement::FruStatement,
    value::fru_value::FruValue,
    value::function::{AnyFunction, FruFunction},
};

#[derive(Debug, Clone)]
pub enum FruExpression {
    Literal(FruValue),
    Variable(Identifier),
    Block {
        body: Vec<FruStatement>,
        expr: Box<FruExpression>,
    },
    Call {
        what: Box<FruExpression>,
        args: Vec<FruExpression>,
    },
    CurryCall {
        what: Box<FruExpression>,
        args: Vec<FruExpression>,
    },
    Binary {
        operator: Identifier,
        left: Box<FruExpression>,
        right: Box<FruExpression>,
    },
    Function {
        args: Vec<Identifier>,
        body: Rc<FruStatement>,
    },
    Instantiation {
        what: Box<FruExpression>,
        args: Vec<FruExpression>,
    },
    FieldAccess {
        what: Box<FruExpression>,
        field: Identifier,
    },
    If {
        condition: Box<FruExpression>,
        then_body: Rc<FruExpression>,
        else_body: Rc<FruExpression>,
    },
}

fn eval_args(args: &[FruExpression], scope: Rc<Scope>) -> Result<Vec<FruValue>, Control> {
    args.iter()
        .map(|arg| arg.evaluate(scope.clone()))
        .try_collect()
}

impl FruExpression {
    pub fn evaluate(&self, mut scope: Rc<Scope>) -> Result<FruValue, Control> {
        match self {
            FruExpression::Literal(value) => Ok(value.clone()),

            FruExpression::Variable(ident) => Ok(scope.get_variable(*ident)?),

            FruExpression::Block { body, expr } => {
                if !body.is_empty() {
                    scope = Scope::new_with_parent(scope.clone())
                };

                for statement in body {
                    statement.execute(scope.clone())?;
                }

                expr.evaluate(scope)
            }

            FruExpression::Call { what, args } => {
                let callee = what.evaluate(scope.clone())?;

                let args = eval_args(args, scope)?;

                Ok(callee.call(args)?)
            }

            FruExpression::CurryCall { what, args } => {
                let callee = what.evaluate(scope.clone())?;

                let args = eval_args(args, scope)?;

                Ok(callee.curry_call(args)?)
            }

            FruExpression::Binary {
                operator,
                left,
                right,
            } => {
                let left_val = left.evaluate(scope.clone())?;
                let right_val = right.evaluate(scope.clone())?;
                let type_left = left_val.get_type_identifier();
                let type_right = right_val.get_type_identifier();

                let op = scope.get_operator(OperatorIdentifier {
                    op: *operator,
                    left: type_left,
                    right: type_right,
                })?;

                Ok(op.operate(left_val, right_val)?)
            }

            FruExpression::Function { args, body } => Ok(FruValue::Function(
                AnyFunction::Function(Rc::new(FruFunction {
                    argument_idents: args.clone(),
                    body: body.clone(),
                    scope: scope.clone(),
                })),
            )),

            FruExpression::Instantiation { what, args } => {
                let instantiated = what.evaluate(scope.clone())?;

                let args = eval_args(args, scope)?;

                Ok(instantiated.instantiate(args)?)
            }

            FruExpression::FieldAccess { what, field } => {
                let what = what.evaluate(scope.clone())?;
                Ok(what.get_field(*field)?)
            }

            FruExpression::If {
                condition,
                then_body,
                else_body,
            } => {
                let condition = condition.evaluate(scope.clone())?;

                if let FruValue::Bool(b) = condition {
                    if b {
                        then_body.evaluate(scope.clone())
                    } else {
                        else_body.evaluate(scope.clone())
                    }
                } else {
                    FruError::new_val_control(format!(
                        "Expected boolean, got {}",
                        condition.get_type_identifier()
                    ))
                }
            }
        }
    }
}
