use thiserror::Error;

use crate::interpreter::value::function::ArgumentError;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct FruError {
    message: String,
}

impl FruError {
    pub fn new(message: String) -> FruError {
        FruError { message }
    }

    pub fn new_res<T>(message: impl Into<String>) -> Result<T, FruError> {
        Err(FruError {
            message: message.into(),
        })
    }
}

impl From<ArgumentError> for FruError {
    fn from(err: ArgumentError) -> Self {
        FruError::new(format!("{:?}", err))
    }
}
