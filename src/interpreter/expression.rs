use std::{path::PathBuf, rc::Rc};

use crate::{
    interpreter::{
        control::Control,
        identifier::Identifier,
        identifier::OperatorIdentifier,
        runner,
        scope::Scope,
        statement::FruStatement,
        value::fru_value::FruValue,
        value::function::{ArgumentList, EvaluatedArgumentList, FormalParameters, FruFunction},
    },
    stdlib::scope::fru_scope::{BScope, extract_scope_from_value},
};
use crate::stdlib::scope::fru_scope::BTypeScope;

#[derive(Debug, Clone)]
pub enum FruExpression {
    Literal {
        value: FruValue,
    },
    Variable {
        ident: Identifier,
    },
    ScopeAccessor,
    Function {
        args: FormalParameters,
        body: Rc<FruStatement>,
    },
    Block {
        body: Vec<FruStatement>,
        expr: Box<FruExpression>,
    },
    ScopeModifier {
        what: Box<FruExpression>,
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
    PropAccess {
        what: Box<FruExpression>,
        ident: Identifier,
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
    Import {
        path: Box<FruExpression>,
    },
}

fn eval_args(args: &ArgumentList, scope: Rc<Scope>) -> Result<EvaluatedArgumentList, Control> {
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
    pub fn evaluate(&self, mut scope: Rc<Scope>) -> Result<FruValue, Control> {
        match self {
            FruExpression::Literal { value } => Ok(value.clone()),

            FruExpression::Variable { ident } => Ok(scope.get_variable(*ident)?),

            FruExpression::ScopeAccessor => Ok(BScope::new_value(scope)),

            FruExpression::Function { args, body } => Ok(FruFunction {
                parameters: args.clone(),
                body: body.clone(),
                scope: scope.clone(),
            }
            .into()),

            FruExpression::Block { body, expr } => {
                scope = Scope::new_with_parent(scope.clone());

                for statement in body {
                    statement.execute(scope.clone())?;
                }

                expr.evaluate(scope)
            }

            FruExpression::ScopeModifier { what, body, expr } => {
                let what = what.evaluate(scope)?;
                if what.get_type() != BTypeScope::get_value() {
                    return Control::new_err(format!(
                        "Expected `Scope` in scope modifier expression, got `{:?}`",
                        what.get_type()
                    ));
                }

                let new_scope = extract_scope_from_value(&what).expect("scope");

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }

                expr.evaluate(new_scope)
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

            FruExpression::PropAccess { what, ident } => {
                let what = what.evaluate(scope.clone())?;

                Ok(what.get_prop(*ident)?)
            }

            FruExpression::Binary {
                operator,
                left,
                right,
            } => {
                let left_val = left.evaluate(scope.clone())?;
                let right_val = right.evaluate(scope.clone())?;
                let type_left = left_val.get_type();
                let type_right = right_val.get_type();

                let op = scope
                    .get_operator(OperatorIdentifier::new(
                        *operator,
                        type_left.get_uid(),
                        type_right.get_uid(),
                    ))
                    .map_err(|x| x.into_error(type_left, type_right))?;

                Ok(op.operate(left_val, right_val)?)
            }

            FruExpression::If {
                condition,
                then_body,
                else_body,
            } => match condition.evaluate(scope.clone())? {
                FruValue::Bool(b) => {
                    if b {
                        then_body.evaluate(scope.clone())
                    } else {
                        else_body.evaluate(scope.clone())
                    }
                }

                unexpected => Control::new_err(format!(
                    "Expected `Bool` in if condition, got `{:?}`",
                    unexpected.get_type()
                )),
            },

            FruExpression::Import { path } => {
                let path = path.evaluate(scope.clone())?;

                let path = match path {
                    FruValue::String(path) => path,

                    _ => {
                        return Control::new_err(format!(
                            "Expected `String` in import path, got `{:?}`",
                            path.get_type()
                        ))
                    }
                };

                let path = PathBuf::from(path);

                let result_scope = runner::execute_file(&path)?;

                Ok(BScope::new_value(result_scope))
            }
        }
    }
}
