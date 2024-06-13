use std::{fs::read_to_string, path::Path, rc::Rc};

use crate::interpreter::{control::Control, error::FruError, scope::Scope, tree_sitter_parser};

pub fn execute_file(path: &Path) -> Result<Rc<Scope>, FruError> {
    let source_code = read_to_string(path)
        .map_err(|err| FruError::new(format!("Error reading file {path:?} {err}")))?;

    let ast = match tree_sitter_parser::parse(source_code) {
        Ok(ast) => ast,
        Err(err) => return Err(FruError::new(err.to_string())),
    };

    let global_scope = Scope::new_global();

    let signal = ast.execute(global_scope.clone());

    if let Err(signal) = signal {
        Err(match signal {
            Control::Error(err) => err,
            unexpected => FruError::new(format!("Unexpected signal: {:?}", unexpected)),
        })
    } else {
        Ok(global_scope)
    }
}
