use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::Control,
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    statement::FruStatement,
    value::fru_value::{FruValue, TFnBuiltin},
    expression::FruExpression,
};

#[derive(Clone, Copy, Debug)]
pub enum ArgumentError {
    TooMany,
    SameSetTwice {
        ident: Identifier,
    },
    NotSetPositional {
        ident: Identifier,
    },
}

#[derive(Clone)]
pub enum AnyFunction {
    Function(Rc<FruFunction>),
    CurriedFunction(Rc<CurriedFunction>),
    BuiltinFunction(BuiltinFunction),
}

#[derive(Clone)]
pub struct FruFunction {
    pub argument_idents: FormalParameters,
    pub body: Rc<FruStatement>,
    pub scope: Rc<Scope>,
}

#[derive(Clone, Debug)]
pub struct FormalParameters {
    pub args: Vec<(Identifier, Option<FruExpression>)>,
    pub minimum_args: usize,
}

#[derive(Clone, Debug)]
pub struct ArgumentList {
    pub args: Vec<(Option<Identifier>, FruExpression)>,
}

#[derive(Clone, Debug)]
pub struct EvaluatedArgumentList {
    pub args: Vec<(Option<Identifier>, FruValue)>,
}

#[derive(Clone)]
pub struct BuiltinFunction {
    pub function: TFnBuiltin,
}

pub struct CurriedFunction {
    pub saved_args: EvaluatedArgumentList,
    pub function: Rc<AnyFunction>,
}

impl AnyFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        match self {
            AnyFunction::Function(func) => func.call(args),
            AnyFunction::BuiltinFunction(func) => func.call(args),
            AnyFunction::CurriedFunction(func) => func.call(args),
        }
    }
}

impl FruFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let new_scope = Scope::new_with_parent(self.scope.clone());

        self.argument_idents.apply(args, new_scope.clone())?;

        let res = self.body.execute(new_scope);
        match res {
            Control::Nah => Ok(FruValue::Nah),
            Control::Return(v) => Ok(v),
            Control::Error(e) => Err(e),
            other => FruError::new_val(format!("unexpected signal {:?}", other)),
        }
    }
}

impl FormalParameters {
    // scope is the scope of function being called
    pub fn apply(&self, evaluated: EvaluatedArgumentList, scope: Rc<Scope>) -> Result<(), FruError> {
        let mut next_positional = 0;

        for (ident, value) in evaluated.args {
            let ident = match ident {
                Some(ident) => ident,
                None => {
                    if next_positional >= self.args.len() {
                        return Err(ArgumentError::TooMany.into());
                    }
                    let r = self.args[next_positional].0;
                    next_positional += 1;
                    r
                }
            };

            scope.let_variable(ident, value).map_err(|_| ArgumentError::SameSetTwice { ident }.into())?;
        }

        for (ident, value) in self.args.iter().skip(next_positional) {
            if scope.has_variable(*ident) {
                continue;
            }

            if let Some(default) = value {
                let default = match default.evaluate(scope.clone()) {
                    Ok(v) => v,

                    Err(Control::Error(err)) => return Err(err),

                    Err(unexpected) => return FruError::new_unit(
                        format!("unexpected signal {:?}", unexpected)
                    ),
                };

                scope.let_variable(*ident, default)?;
            } else {
                return Err(ArgumentError::NotSetPositional { ident: *ident }.into());
            }
        }

        Ok(())
    }
}

impl BuiltinFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        (self.function)(args)
    }
}

impl CurriedFunction {
    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let mut new_args = self.saved_args.clone();
        new_args.args.extend(args.args);

        match *self.function {
            AnyFunction::Function(ref func) => func.call(new_args),
            AnyFunction::BuiltinFunction(ref func) => func.call(new_args),
            AnyFunction::CurriedFunction(_) => {
                unreachable!("CurriedFunction should never contain a CurriedFunction")
            }
        }
    }
}

impl Into<FruError> for ArgumentError {
    fn into(self) -> FruError {
        FruError::new(
            format!("{:?}", self)
        )
    }
}

impl Debug for FruFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

impl Debug for AnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyFunction::Function(_) | AnyFunction::BuiltinFunction(_) => write!(f, "Function"),
            AnyFunction::CurriedFunction(func) => {
                write!(
                    f,
                    "CurriedFunction({})",
                    func.saved_args.args.len(),
                )
            }
        }
    }
}
