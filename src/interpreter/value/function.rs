use std::{cmp::Ordering, fmt::Debug, ops::Sub, rc::Rc};

use crate::interpreter::{
    control::Control,
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    statement::FruStatement,
    value::fru_value::{FruValue, TFnBuiltin},
};

#[derive(Debug, Clone, Copy)]
pub enum ArgCount {
    Exact(usize),
    AtMost(usize),
    Any,
}

#[derive(Clone, Copy)]
pub enum ArgCountError {
    TooManyArgs { expected: ArgCount, got: usize },
    TooFewArgs { expected: ArgCount, got: usize },
}

#[derive(Clone)]
pub enum AnyFunction {
    Function(Rc<FruFunction>),
    CurriedFunction(Rc<CurriedFunction>),
    BuiltinFunction(BuiltinFunction),
}

#[derive(Clone)]
pub struct FruFunction {
    pub argument_idents: Vec<Identifier>,
    pub body: Rc<FruStatement>,
    pub scope: Rc<Scope>,
}

#[derive(Clone)]
pub struct BuiltinFunction {
    pub function: TFnBuiltin,
    pub argument_count: ArgCount,
}

pub struct CurriedFunction {
    pub saved_args: Vec<FruValue>,
    pub function: Rc<AnyFunction>,
}

impl ArgCount {
    pub fn satisfies(&self, got: usize) -> Result<(), ArgCountError> {
        match self {
            ArgCount::Exact(n) => match got.cmp(n) {
                Ordering::Equal => Ok(()),
                Ordering::Greater => Err(ArgCountError::TooManyArgs {
                    expected: *self,
                    got,
                }),
                Ordering::Less => Err(ArgCountError::TooFewArgs {
                    expected: *self,
                    got,
                }),
            },

            ArgCount::AtMost(n) => {
                if got <= *n {
                    Ok(())
                } else {
                    Err(ArgCountError::TooManyArgs {
                        expected: *self,
                        got,
                    })
                }
            }

            ArgCount::Any => Ok(()),
        }
    }
}

impl Sub<usize> for ArgCount {
    type Output = ArgCount;

    fn sub(self, rhs: usize) -> Self::Output {
        match self {
            ArgCount::Exact(n) => ArgCount::Exact(n.checked_sub(rhs).unwrap()),
            ArgCount::AtMost(n) => ArgCount::AtMost(n.checked_sub(rhs).unwrap()),
            ArgCount::Any => ArgCount::Any,
        }
    }
}

impl ArgCountError {
    pub fn to_error(self) -> FruError {
        FruError::new(format!("{:?}", self))
    }
}

impl AnyFunction {
    pub fn call(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        if let Err(err) = self.get_arg_count().satisfies(args.len()) {
            return Err(err.to_error());
        }

        match self {
            AnyFunction::Function(func) => func.call_unchecked(args),
            AnyFunction::BuiltinFunction(func) => func.call_unchecked(args),
            AnyFunction::CurriedFunction(func) => func.call_unchecked(args),
        }
    }

    pub fn get_arg_count(&self) -> ArgCount {
        match self {
            AnyFunction::Function(func) => func.get_arg_count(),
            AnyFunction::BuiltinFunction(func) => func.get_arg_count(),
            AnyFunction::CurriedFunction(func) => func.get_arg_count(),
        }
    }
}

impl FruFunction {
    pub fn call_unchecked(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        let new_scope = Scope::new_with_parent(self.scope.clone());

        for (ident, value) in self.argument_idents.iter().zip(args.iter()) {
            new_scope
                .let_variable(*ident, value.clone())
                .expect("should NEVER happen XD :)");
        }

        let res = self.body.execute(new_scope);
        match res {
            Control::Nah => Ok(FruValue::Nah),
            Control::Return(v) => Ok(v),
            Control::Error(e) => Err(e),
            other => FruError::new_val(format!("unexpected signal {:?}", other)),
        }
    }

    pub fn get_arg_count(&self) -> ArgCount {
        ArgCount::Exact(self.argument_idents.len())
    }
}

impl BuiltinFunction {
    pub fn call_unchecked(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        (self.function)(args)
    }

    pub fn get_arg_count(&self) -> ArgCount {
        self.argument_count
    }
}

impl CurriedFunction {
    pub fn call_unchecked(&self, args: Vec<FruValue>) -> Result<FruValue, FruError> {
        let mut new_args = self.saved_args.clone();
        new_args.extend(args);

        match *self.function {
            AnyFunction::Function(ref func) => func.call_unchecked(new_args),
            AnyFunction::BuiltinFunction(ref func) => func.call_unchecked(new_args),
            AnyFunction::CurriedFunction(_) => {
                unreachable!("CurriedFunction should never contain a CurriedFunction")
            }
        }
    }

    pub fn get_arg_count(&self) -> ArgCount {
        let internal = match *self.function {
            AnyFunction::Function(ref func) => func.get_arg_count(),
            AnyFunction::BuiltinFunction(ref func) => func.get_arg_count(),
            _ => unreachable!("CurriedFunction should never contain a CurriedFunction"),
        };

        internal - self.saved_args.len()
    }
}

impl Debug for ArgCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgCountError::TooManyArgs { expected, got } => {
                write!(
                    f,
                    "too many arguments, expected {:?}, got {}",
                    expected, got
                )
            }

            ArgCountError::TooFewArgs { expected, got } => {
                write!(f, "too few arguments, expected {:?}, got {}", expected, got)
            }
        }
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
                    "CurriedFunction({}/{:?})",
                    func.saved_args.len(),
                    func.function.get_arg_count()
                )
            }
        }
    }
}
