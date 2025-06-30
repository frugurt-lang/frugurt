use std::rc::Rc;

use crate::{
    common::{
        ArgumentList, Control, EvaluatedArgumentList, FormalParameters, FruError, FruFunction,
        FruStatement, FruValue, Identifier, OperatorIdentifier, Thing,
    },
    fru_err,
};

#[derive(Debug, Clone)]
pub enum FruExpression {
    Literal {
        value: FruValue,
    },
    Variable {
        ident: Identifier,
    },
    ObjectCapture,
    Function {
        args: FormalParameters,
        body: Rc<FruStatement>,
    },
    Block {
        body: Vec<FruStatement>,
        expr: Box<FruExpression>,
    },
    ObjectAlter {
        what: Box<FruExpression>,
        body: Vec<FruStatement>,
        expr: Box<FruExpression>,
    },
    Call {
        what: Box<FruExpression>,
        args: ArgumentList,
    },
    Index {
        what: Box<FruExpression>,
        args: ArgumentList,
    },
    GetProp {
        what: Box<FruExpression>,
        ident: Identifier,
    },
    If {
        condition: Box<FruExpression>,
        then_body: Box<FruExpression>,
        else_body: Box<FruExpression>,
    },
    Set {
        ident: Identifier,
        value: Box<FruExpression>,
    },
    SetProp {
        what: Box<FruExpression>,
        ident: Identifier,
        value: Box<FruExpression>,
    },
    LetProp {
        what: Box<FruExpression>,
        ident: Identifier,
        value: Box<FruExpression>,
    },
    Binary {
        operator: Identifier,
        left: Box<FruExpression>,
        right: Box<FruExpression>,
    },
}

fn eval_args(args: &ArgumentList, scope: Thing) -> Result<EvaluatedArgumentList, Control> {
    Ok(EvaluatedArgumentList {
        args: args
            .args
            .iter()
            .map(|(ident, arg)| -> Result<_, Control> {
                Ok((*ident, arg.evaluate(scope.clone())?))
            })
            .collect::<Result<_, _>>()?,
    })
}

impl FruExpression {
    pub fn evaluate(&self, scope: Thing) -> Result<FruValue, Control> {
        match self {
            FruExpression::Literal { value } => Ok(value.clone()),

            FruExpression::Variable { ident } => Ok(scope.get_prop(*ident)?),

            FruExpression::ObjectCapture => Ok(FruValue::Thing(scope)),

            FruExpression::Function { args, body } => {
                Ok(FruValue::Function(Rc::new(FruFunction {
                    parameters: args.clone(),
                    body: body.clone(),
                    scope: scope.clone(),
                })))
            }

            FruExpression::Block { body, expr } => {
                let new_scope = scope.derive_new();

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }

                expr.evaluate(new_scope)
            }

            FruExpression::ObjectAlter { what, body, expr } => {
                let new_scope = match what.evaluate(scope)? {
                    FruValue::Thing(new_scope) => new_scope,

                    unexpected => {
                        return Control::new_err(format!(
                            "Only `Thing` can be altered, not {:?}",
                            unexpected
                        ));
                    }
                };

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }

                expr.evaluate(new_scope)
            }

            FruExpression::Call { what, args } => {
                let obj = what.evaluate(scope.clone())?;
                let args = eval_args(args, scope)?;
                Ok(obj.call(args)?)
            }

            FruExpression::Index { what, args } => {
                let obj = what.evaluate(scope.clone())?;
                let args = eval_args(args, scope)?;
                Ok(obj.index(args)?)
            }

            FruExpression::GetProp { what, ident } => {
                let what = what.evaluate(scope.clone())?;
                Ok(what.get_prop(*ident)?)
            }

            FruExpression::SetProp { what, ident, value } => {
                let obj = what.evaluate(scope.clone())?;
                let val = value.evaluate(scope.clone())?;
                obj.set_prop(*ident, val.clone())?;
                Ok(val)
            }

            FruExpression::LetProp { what, ident, value } => {
                let t = what.evaluate(scope.clone())?;
                let v = value.evaluate(scope.clone())?;
                t.set_prop(*ident, v.clone())?;
                Ok(v)
            }

            FruExpression::Set { ident, value } => {
                let val = value.evaluate(scope.clone())?;
                scope.set_prop(*ident, val.clone())?;
                Ok(val)
            }

            FruExpression::If {
                condition,
                then_body,
                else_body,
            } => match condition.evaluate(scope.clone())? {
                FruValue::Bool(b) => {
                    if b {
                        then_body.evaluate(scope)
                    } else {
                        else_body.evaluate(scope)
                    }
                }

                unexpected => Control::new_err(format!(
                    "Expected `Bool` in if condition, got `{:?}`",
                    unexpected
                )),
            },

            FruExpression::Binary {
                operator,
                left,
                right,
            } => {
                let val_left = left.evaluate(scope.clone())?;
                let val_right = right.evaluate(scope.clone())?;
                let type_left = val_left.get_type_uid();
                let type_right = val_right.get_type_uid();

                let op = type_left.get_operator(OperatorIdentifier::new()).ok_or_else(|| {
                    fru_err!(
                        "operator `{:?}` between `{:?}` and `{:?}` does not exist",
                        operator,
                        type_left,
                        type_right
                    )
                })?;

                op.operate(val_left, val_right).map_err(Into::into)
            }
        }
    }
}
