use crate::interpreter::{error::FruError, value::fru_value::FruValue};

#[derive(Debug)]
pub enum Control {
    Continue,
    Break,
    Return(FruValue),
    Error(FruError),
}


impl From<FruError> for Control {
    fn from(err: FruError) -> Self {
        Control::Error(err)
    }
}
