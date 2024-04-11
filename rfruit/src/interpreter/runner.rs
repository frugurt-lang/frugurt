use std::{env::current_dir, path::Path, process::Command, rc::Rc};

use serde_json::Value;

use crate::interpreter::{ast_json_parser, control::Control, error::FruError, scope::Scope};

pub fn execute_file(path: &Path, converter_path: Option<&Path>) -> Result<Rc<Scope>, FruError> {
    let converter_result = Command::new(
        converter_path.unwrap_or(
            current_dir()
                .unwrap()
                .join(Path::new("converter"))
                .as_path(),
        ),
    )
    .arg(path)
    .output()
    .expect("converter should be located in the same directory");

    let text = std::str::from_utf8(&converter_result.stdout);

    let text = match text {
        Ok(text) => text,
        Err(error) => return Err(FruError::new(format!("Parser error: {error}"))),
    };

    let json_ast = serde_json::from_str(text);

    let json_ast: Value = match json_ast {
        Ok(json_ast) => json_ast,
        Err(error) => return Err(FruError::new(format!("Serde error: {error} {text}"))),
    };

    if let Some(x) = json_ast.get("error") {
        return Err(FruError::new(format!(
            "Error occurred during parsing: {}\n\
            Details: {}",
            x.as_str().unwrap(),
            json_ast["message"].as_str().unwrap()
        )));
    }

    let ast = ast_json_parser::parse(json_ast);

    let global_scope = Scope::new_global();

    let result = ast.execute(global_scope.clone());

    match result {
        Control::Nah => {}
        Control::Error(e) => return Err(e),
        unexpected_signal => {
            return Err(FruError::new(format!(
                "Unexpected signal: {:?}",
                unexpected_signal
            )))
        }
    }

    Ok(global_scope)
}
