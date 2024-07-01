use thiserror::Error;

use crate::interpreter::value::function_helpers::ArgumentError;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct FruError {
    message: String,
}

impl FruError {
    pub fn new(message: impl ToString) -> FruError {
        FruError {
            message: message.to_string(),
        }
    }

    pub fn new_res<T>(message: impl ToString) -> Result<T, FruError> {
        Err(FruError {
            message: message.to_string(),
        })
    }
}

impl From<ArgumentError> for FruError {
    fn from(err: ArgumentError) -> Self {
        FruError::new(format!("{:?}", err))
    }
}

#[macro_export]
macro_rules! fru_err {
    ($($content:tt)*) => {
        FruError::new(format!($($content)*))
    };
}
#[macro_export]
macro_rules! fru_err_res {
    ($($content:tt)*) => {
        FruError::new_res(format!($($content)*))
    };
}
