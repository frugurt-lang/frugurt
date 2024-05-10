use crate::interpreter::{error::FruError, value::fru_value::FruValue};

#[derive(Debug)]
pub enum Control {
    Continue,
    Break,
    Return(FruValue),
    Error(FruError),
}

impl Control {
    pub fn new_err<T>(message: impl Into<String>) -> Result<T, Control> {
        Err(Control::Error(FruError::new(message.into())))
    }
}

impl From<FruError> for Control {
    fn from(err: FruError) -> Self {
        Control::Error(err)
    }
}

pub fn returned(x: Result<FruValue, Control>) -> Result<FruValue, FruError> {
    match x {
        Ok(x) => Ok(x),
        Err(Control::Return(x)) => Ok(x),
        Err(Control::Error(err)) => Err(err),
        Err(unexpected) => FruError::new_res(format!("unexpected signal {:?}", unexpected)),
    }
}

pub fn returned_unit(x: Result<(), Control>) -> Result<FruValue, FruError> {
    match x {
        Ok(()) => Ok(FruValue::Nah),
        Err(Control::Return(x)) => Ok(x),
        Err(Control::Error(err)) => Err(err),
        Err(unexpected) => FruError::new_res(format!("unexpected signal {:?}", unexpected)),
    }
}

pub fn returned_nothing(x: Result<(), Control>) -> Result<(), FruError> {
    match x {
        Ok(()) => Ok(()),
        Err(Control::Return(FruValue::Nah)) => Ok(()),
        Err(Control::Error(err)) => Err(err),
        Err(unexpected) => FruError::new_res(format!("unexpected signal {:?}", unexpected)),
    }
}
