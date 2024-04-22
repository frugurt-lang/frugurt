use std::{path::Path, rc::Rc};

use crate::interpreter::{control::Control, error::FruError, scope::Scope, tree_sitter_parser};

pub fn execute_file(path: &Path) -> Result<Rc<Scope>, FruError> {
    let source_code = std::fs::read_to_string(path).unwrap();

    let ast = tree_sitter_parser::parse(source_code).unwrap();

    let global_scope = Scope::new_global();

    let result = ast.execute(global_scope.clone());

    match result {
        Control::Nah => {}
        Control::Error(e) => return Err(e),
        unexpected_signal => return Err(FruError::new(format!(
            "Unexpected signal: {:?}",
            unexpected_signal
        )))
    }

    Ok(global_scope)
}
