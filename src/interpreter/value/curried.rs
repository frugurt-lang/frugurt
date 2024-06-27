use std::{fmt::Debug, rc::Rc};

use crate::interpreter::{
    error::FruError,
    value::{fru_value::FruValue, function_helpers::EvaluatedArgumentList},
};

pub struct Curried {
    saved_args: EvaluatedArgumentList,
    what: FruValue,
}

impl Curried {
    pub fn new_value(what: FruValue, args: EvaluatedArgumentList) -> FruValue {
        FruValue::Curried(Rc::new(Curried {
            saved_args: args,
            what,
        }))
    }

    pub fn call(&self, args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let mut new_args = self.saved_args.clone();
        new_args.args.extend(args.args);

        self.what.call(new_args)
    }

    pub fn curry_call(&self, args: EvaluatedArgumentList) -> FruValue {
        let mut new_args = self.saved_args.clone(); // TODO: obsidian Issue 1
        new_args.args.extend(args.args);

        FruValue::Curried(Rc::new(Curried {
            saved_args: new_args,
            what: self.what.clone(),
        }))
    }
}

impl Debug for Curried {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Curried")
    }
}
