use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
};

use crate::interpreter::{error::FruError, value::fru_value::FruValue};

#[derive(Debug)]
pub enum Control {
    Nah,
    Continue,
    Break,
    Return(FruValue),
    Error(FruError),
}

impl FromResidual<Control> for Result<FruValue, Control> {
    fn from_residual(residual: Control) -> Self {
        Err(residual)
    }
}

impl FromResidual<Control> for Control {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl FromResidual<Result<Infallible, Control>> for Control {
    fn from_residual(residual: Result<Infallible, Control>) -> Self {
        match residual {
            Ok(_) => unreachable!(), // TODO: check
            Err(v) => v,
        }
    }
}

impl FromResidual<Result<Infallible, FruError>> for Control {
    fn from_residual(residual: Result<Infallible, FruError>) -> Self {
        match residual {
            Ok(_) => unreachable!(), // TODO: check
            Err(v) => Control::Error(v),
        }
    }
}

impl Try for Control {
    type Output = Control;
    type Residual = Control;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Control::Nah => ControlFlow::Continue(Control::Nah),
            _ => ControlFlow::Break(self),
        }
    }
}

impl From<FruError> for Control {
    fn from(err: FruError) -> Self {
        Control::Error(err)
    }
}
