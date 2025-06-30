use std::{cell::RefCell, collections::HashMap};

use crate::{
    common::{fru_err_res, Control, FruError, Operator, OperatorIdentifier, Thing},
    stdlib::common::{builtin_functions, builtin_operators, builtin_variables},
};

pub struct Interpreter {
    global_scope: Thing,
    operators: RefCell<HashMap<OperatorIdentifier, Operator>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let res = Self {
            global_scope: Thing::new(),
            operators: Default::default(),
        };

        builtin_functions(res.global_scope.clone()).unwrap();
        builtin_variables(res.global_scope.clone()).unwrap();
        builtin_operators(&mut *res.operators.borrow_mut());
        res
    }

    pub fn execute_code(&self, source_code: String) -> Result<(), FruError> {
        let ast = match crate::tree_sitter_parser::parse(source_code) {
            Ok(ast) => ast,
            Err(err) => return Err(FruError::new(err.to_string())),
        };

        let signal = ast.execute(self.global_scope.derive_new());

        match signal {
            Ok(()) => Ok(()),
            Err(Control::Error(err)) => Err(err),
            Err(unexpected) => fru_err_res!("Unexpected signal: {:?}", unexpected),
        }
    }
}
