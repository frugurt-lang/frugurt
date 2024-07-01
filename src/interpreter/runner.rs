use std::{fs::read_to_string, path::Path, rc::Rc};

use crate::{
    fru_err, fru_err_res,
    interpreter::{control::Control, error::FruError, scope::Scope, tree_sitter_parser},
};

pub fn execute_file(path: &Path) -> Result<Rc<Scope>, FruError> {
    let source_code =
        read_to_string(path).map_err(|err| fru_err!("Error reading file {path:?} {err}"))?;

    execute_source_code(source_code)
}

pub fn execute_source_code(source_code: String) -> Result<Rc<Scope>, FruError> {
    let ast = match tree_sitter_parser::parse(source_code) {
        Ok(ast) => ast,
        Err(err) => return Err(FruError::new(err.to_string())),
    };

    let global_scope = Scope::new_global();

    let signal = ast.execute(global_scope.clone());

    match signal {
        Ok(_) => Ok(global_scope),

        Err(Control::Error(err)) => Err(err),
        Err(unexpected) => fru_err_res!("Unexpected signal: {:?}", unexpected),
    }
}
