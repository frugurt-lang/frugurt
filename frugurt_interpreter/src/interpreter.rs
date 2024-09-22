use std::rc::Rc;

use crate::common::*;

pub struct Interpreter {
    pub global_scope: Rc<Scope>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            global_scope: Scope::new_global(),
        }
    }

    pub fn execute_code(self, source_code: String) -> Result<(), FruError> {
        let ast = match crate::tree_sitter_parser::parse(source_code) {
            Ok(ast) => ast,
            Err(err) => return Err(FruError::new(err.to_string())),
        };

        let signal = ast.execute(Scope::new_with_parent(self.global_scope));

        match signal {
            Ok(()) => Ok(()),
            Err(Control::Error(err)) => Err(err),
            Err(unexpected) => fru_err_res!("Unexpected signal: {:?}", unexpected),
        }
    }
}
