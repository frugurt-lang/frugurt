use std::rc::Rc;

use crate::interpreter::{
    control::Control, error::FruError, scope::Scope, statement::FruStatement, value::fru_value::FruValue,
};

#[derive(Clone)]
pub struct FruWatch {
    pub body: Rc<FruStatement>,
}

impl FruWatch {
    pub fn run(&self, scope: Rc<Scope>) -> Result<(), FruError> {
        let signal = self.body.execute(scope);

        if let Err(signal) = signal {
            match signal {
                Control::Return(FruValue::Nah) => Ok(()),
                Control::Return(v) => {
                    FruError::new_unit(format!("watch returned {:?}, but should be None", v))
                }
                Control::Error(e) => Err(e),
                other => FruError::new_unit(format!("unexpected signal {:?}", other)),
            }
        } else {
            Ok(())
        }
    }
}
