use std::{path::PathBuf, rc::Rc};

use crate::{
    interpreter::{
        control::Control,
        error::FruError,
        identifier::{Identifier, OperatorIdentifier},
        runner,
        scope::Scope,
        statement::FruStatement,
        value::{
            fru_function::FruFunction,
            fru_value::FruValue,
            function_helpers::{ArgumentList, EvaluatedArgumentList, FormalParameters},
            native_object::cast_object,
        },
    },
    stdlib::prelude::{
        builtin_scope_instance::BuiltinScopeInstance,
        builtin_string_instance::BuiltinStringInstance,
    },
};

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

            FruExpression::ScopeAccessor => Ok(BuiltinScopeInstance::new_value(scope)),

            FruExpression::Function { args, body } => {
                Ok(FruValue::Function(Rc::new(FruFunction {
                    parameters: args.clone(),
                    body: body.clone(),
                    scope: scope.clone(),
                })))
            }

            FruExpression::Block { body, expr } => {
                scope = Scope::new_with_parent(scope.clone());

                for statement in body {
                    statement.execute(scope.clone())?;
                }

                expr.evaluate(scope)
            }

            FruExpression::ScopeModifier { what, body, expr } => {
                let what = what.evaluate(scope)?;

                let new_scope = match cast_object::<BuiltinScopeInstance>(&what) {
                    Some(new_scope) => new_scope.scope.clone(),
                    None => {
                        return Control::new_err(format!(
                            "Expected `Scope` in scope modifier expression, got `{:?}`",
                            what.get_type()
                        ));
                    }
                };

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

                let op = type_left
                    .get_operator(OperatorIdentifier::new(*operator, type_right.get_uid()))
                    .ok_or_else(|| {
                        FruError::new(format!(
                            "operator `{:?}` between `{:?}` and `{:?}` does not exist",
                            operator, type_left, type_right
                        ))
                    })?;

                op.operate(left_val, right_val).map_err(Into::into)
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
                    unexpected.get_type()
                )),
            },

            FruExpression::Import { path } => {
                let path = path.evaluate(scope.clone())?;

                let path = match cast_object::<BuiltinStringInstance>(&path) {
                    Some(path) => path.value.clone(),

                    _ => {
                        return Control::new_err(format!(
                            "Expected `String` in import path, got `{:?}`",
                            path.get_type()
                        ))
                    }
                };

                let path = PathBuf::from(path);

                let result_scope = runner::execute_file(&path)?;

                Ok(BuiltinScopeInstance::new_value(result_scope))
            }
        }
    }
}
