use thiserror::Error; // TODO: make use of this

use crate::interpreter::{control::Control, value::fru_value::FruValue};

#[derive(Debug, Error)]
#[error("{message}")]
pub struct FruError {
    message: String,
}

impl FruError {
    pub fn new(message: String) -> FruError {
        FruError { message }
    }

    pub fn new_val(message: String) -> Result<FruValue, FruError> {
        Err(FruError { message })
    }

    pub fn new_val_slice(message: &str) -> Result<FruValue, FruError> {
        Err(FruError {
            message: message.to_string(),
        })
    }

    pub fn new_unit(message: String) -> Result<(), FruError> {
        Err(FruError { message })
    }

    pub fn new_unit_slice(message: &str) -> Result<(), FruError> {
        Err(FruError {
            message: message.to_string(),
        })
    }

    pub fn new_control(message: String) -> Result<(), Control> {
        Err(Control::Error(FruError { message }))
    }

    pub fn new_val_control(message: String) -> Result<FruValue, Control> {
        Err(Control::Error(FruError { message }))
    }
}
