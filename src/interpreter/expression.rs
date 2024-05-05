use std::rc::Rc;

use crate::interpreter::{
    control::Control,
    error::FruError,
    identifier::{Identifier, OperatorIdentifier},
    scope::Scope,
    statement::FruStatement,
    value::fru_value::FruValue,
    value::function::{AnyFunction, FruFunction, ArgumentList, EvaluatedArgumentList, FormalParameters},
};

#[derive(Debug, Clone)]
pub enum FruExpression {
    Literal {
        value: FruValue
    },
    Variable {
        ident: Identifier
    },
    Function {
        args: FormalParameters,
        body: Rc<FruStatement>,
    },
    Block {
        body: Vec<FruStatement>,
        expr: Box<FruExpression>,
    },
    Call {
        what: Box<FruExpression>,
        args: ArgumentList,
    },
    CurryCall {
        what: Box<FruExpression>,
        args: ArgumentList,
    },
    Instantiation {
        what: Box<FruExpression>,
        args: ArgumentList,
    },
    FieldAccess {
        what: Box<FruExpression>,
        field: Identifier,
    },
    Binary {
        operator: Identifier,
        left: Box<FruExpression>,
        right: Box<FruExpression>,
    },
    If {
        condition: Box<FruExpression>,
        then_body: Box<FruExpression>,
        else_body: Box<FruExpression>,
    },
}

fn eval_args(args: &ArgumentList, scope: Rc<Scope>) -> Result<EvaluatedArgumentList, Control> {
    Ok(EvaluatedArgumentList {
        args: args.args.iter().map(
            |(ident, arg)| -> Result<_, Control>{
                Ok((*ident, arg.evaluate(scope.clone())?))
            }
        ).try_collect()?
    })
}

impl FruExpression {
    pub fn evaluate(&self, mut scope: Rc<Scope>) -> Result<FruValue, Control> {
        match self {
            FruExpression::Literal { value } => Ok(value.clone()),

            FruExpression::Variable { ident } => Ok(scope.get_variable(*ident)?),

            FruExpression::Function { args, body } => {
                Ok(FruValue::Function(AnyFunction::Function(
                    Rc::new(FruFunction {
                        argument_idents: args.clone(),
                        body: body.clone(),
                        scope: scope.clone(),
                    })
                )))
            }

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

            FruExpression::Instantiation { what, args } => {
                let instantiated = what.evaluate(scope.clone())?;

                let args = eval_args(args, scope)?;

                Ok(instantiated.instantiate(args)?)
            }

            FruExpression::FieldAccess { what, field } => {
                let what = what.evaluate(scope.clone())?;

                Ok(what.get_field(*field)?)
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

            FruExpression::If {
                condition,
                then_body,
                else_body,
            } => {
                match condition.evaluate(scope.clone())? {
                    FruValue::Bool(b) => {
                        if b {
                            then_body.evaluate(scope.clone())
                        } else {
                            else_body.evaluate(scope.clone())
                        }
                    }

                    unexpected => FruError::new_val_control(format!(
                        "Expected boolean, got {}",
                        unexpected.get_type_identifier()
                    )),
                }
            }
        }
    }
}
