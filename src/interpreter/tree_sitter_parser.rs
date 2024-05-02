use std::{
    boxed::Box,
    rc::Rc,
    str::Utf8Error,
};

use snailquote::unescape;
use thiserror::Error;
use tree_sitter::{Node, Parser, Range};
use tree_sitter_frugurt;

use crate::helpers::WrappingExtension;
use crate::interpreter::{
    expression::FruExpression,
    identifier::Identifier,
    statement::FruStatement,
    value::fru_type::FruField,
    value::fru_type::TypeType,
    value::fru_value::FruValue,
};

#[derive(Error, Debug)]
pub enum ParseError {
    // in theory could never happen
    #[error("node {} is not provided by tree-sitter-frugurt at {}:{}-{}:{}",
    name,
    .position.start_point.row + 1,
    .position.start_point.column,
    .position.end_point.row + 1,
    .position.end_point.column)]
    MissingAst {
        position: Range,
        name: String,
    },

    // in theory could never happen
    #[error("{} at {}:{}-{}:{}",
    .error,
    .position.start_point.row + 1,
    .position.start_point.column,
    .position.end_point.row + 1,
    .position.end_point.column)]
    InvalidAst {
        position: Range,
        error: String,
    },

    // I don't know if this can ever happen
    #[error("utf8 decoding error at {}:{}-{}:{}",
    .position.start_point.row + 1,
    .position.start_point.column,
    .position.end_point.row + 1,
    .position.end_point.column)]
    Utf8Error {
        position: Range,
        error: Utf8Error,
    },

    // error that occurred in tree-sitter-frugurt library
    #[error("parsing error at {}:{}-{}:{}",
    .position.start_point.row + 1,
    .position.start_point.column,
    .position.end_point.row + 1,
    .position.end_point.column)]
    ParsingError {
        position: Range,
    },
}

enum TypeSection {
    Impl(Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>),
    Static(Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>),
    Constraints(Vec<(Vec<Identifier>, Rc<FruStatement>)>),
}

enum AnyField {
    Normal(FruField),
    Static((FruField, Option<Box<FruExpression>>)),
}

trait NodeExtension {
    fn get_child(&self, name: &str) -> Result<Node, ParseError>;

    fn text<'a>(&self, source: &'a [u8]) -> Result<&'a str, ParseError>;

    fn parse_statement(self, source: &[u8]) -> Result<FruStatement, ParseError>;

    fn parse_expression(self, source: &[u8]) -> Result<FruExpression, ParseError>;

    fn parse_function_body(self, source: &[u8]) -> Result<FruStatement, ParseError>;

    fn parse_field(self, source: &[u8]) -> Result<AnyField, ParseError>;

    fn parse_section(self, source: &[u8]) -> Result<TypeSection, ParseError>;

    fn parse_method(self, source: &[u8]) -> Result<(Identifier, Vec<Identifier>, Rc<FruStatement>), ParseError>;

    fn parse_watch(self, source: &[u8]) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError>;
}

impl NodeExtension for Node<'_> {
    fn get_child(&self, name: &str) -> Result<Node, ParseError> {
        match self.child_by_field_name(name) {
            Some(x) => Ok(x),
            None => Err(ParseError::MissingAst {
                position: self.range(),
                name: name.to_string(),
            })
        }
    }

    fn text<'a>(&self, source: &'a [u8]) -> Result<&'a str, ParseError> {
        match self.utf8_text(source) {
            Ok(x) => Ok(x),
            Err(e) => Err(ParseError::Utf8Error {
                position: self.range(),
                error: e,
            })
        }
    }

    fn parse_statement(self, source: &[u8]) -> Result<FruStatement, ParseError> {
        parse_statement(self, source)
    }

    fn parse_expression(self, source: &[u8]) -> Result<FruExpression, ParseError> {
        parse_expression(self, source)
    }

    fn parse_function_body(self, source: &[u8]) -> Result<FruStatement, ParseError> {
        parse_function_body(self, source)
    }

    fn parse_field(self, source: &[u8]) -> Result<AnyField, ParseError> {
        parse_field(self, source)
    }

    fn parse_section(self, source: &[u8]) -> Result<TypeSection, ParseError> {
        parse_section(self, source)
    }

    fn parse_method(self, source: &[u8]) -> Result<(Identifier, Vec<Identifier>, Rc<FruStatement>), ParseError> {
        parse_method(self, source)
    }

    fn parse_watch(self, source: &[u8]) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError> {
        parse_watch(self, source)
    }
}


trait IntoIdentifierExtension {
    fn to_ident(self) -> Identifier;
}

impl IntoIdentifierExtension for &str {
    fn to_ident(self) -> Identifier {
        Identifier::new(self)
    }
}

trait OptionalParsingExtension {
    fn parse_optional_statement(self, source: &[u8]) -> Result<Option<FruStatement>, ParseError>;

    fn parse_optional_expression(self, source: &[u8]) -> Result<Option<FruExpression>, ParseError>;
}

impl OptionalParsingExtension for Result<Node<'_>, ParseError> {
    fn parse_optional_statement(self, source: &[u8]) -> Result<Option<FruStatement>, ParseError> {
        Ok(match self {
            Ok(x) => Some(x.parse_statement(source)?),

            Err(ParseError::MissingAst { .. }) => None,

            Err(err) => return Err(err)
        })
    }
    fn parse_optional_expression(self, source: &[u8]) -> Result<Option<FruExpression>, ParseError> {
        Ok(match self {
            Ok(x) => Some(x.parse_expression(source)?),

            Err(ParseError::MissingAst { .. }) => None,

            Err(err) => return Err(err)
        })
    }
}


pub fn parse(data: String) -> Result<Box<FruStatement>, ParseError> {
    let source = data.as_bytes();

    let mut parser = Parser::new();

    parser // TODO: load grammar one time
        .set_language(&tree_sitter_frugurt::language())
        .expect("Error loading Frugurt grammar");

    let tree = parser.parse(source, None).unwrap();

    let root = tree.root_node();

    if root.has_error() {
        return Err(search_for_errors(root));
    }

    root.parse_statement(source).map(Box::new)
}

fn search_for_errors(ast: Node) -> ParseError {
    let mut cur = ast.walk();

    loop {
        if cur.node().is_error() {
            return ParseError::ParsingError {
                position: cur.node().range(),
            };
        }

        if cur.node().has_error() {
            cur.goto_first_child();
            continue;
        } else {
            cur.goto_next_sibling();
        }
    }
}

fn parse_statement(ast: Node, source: &[u8]) -> Result<FruStatement, ParseError> {
    let result_statement = match ast.grammar_name() {
        "source_file" => FruStatement::Block {
            body: ast.children_by_field_name("body", &mut ast.walk())
                     .map(|x| x.parse_statement(source))
                     .try_collect()?
        },

        "block_statement" => FruStatement::Block {
            body: ast.children_by_field_name("body", &mut ast.walk())
                     .map(|x| x.parse_statement(source))
                     .try_collect()?
        },

        "expression_statement" => FruStatement::Expression {
            value: ast.get_child("value")?.parse_expression(source)?.wrap_box(),
        },

        "let_statement" => FruStatement::Let {
            ident: ast.get_child("ident")?.text(source)?.to_ident(),
            value: ast.get_child("value")?.parse_expression(source)?.wrap_box(),
        },

        "set_statement" => FruStatement::Set {
            ident: ast.get_child("ident")?.text(source)?.to_ident(),
            value: ast.get_child("value")?.parse_expression(source)?.wrap_box(),
        },

        "set_field_statement" => {
            match ast.get_child("what")?.parse_expression(source)? {
                FruExpression::FieldAccess { what, field } => {
                    FruStatement::SetField {
                        what,
                        field,
                        value: ast.get_child("value")?.parse_expression(source)?.wrap_box(),
                    }
                }

                unexpected => return Err(ParseError::InvalidAst {
                    position: ast.range(),
                    error: format!("set_field_statement: what is not a field access {:?}", unexpected),
                })
            }
        }

        "if_statement" => {
            FruStatement::If {
                condition: ast.get_child("condition")?.parse_expression(source)?.wrap_box(),
                then_body: ast.get_child("then_body")?.parse_statement(source)?.wrap_box(),
                else_body: ast.get_child("else_body").parse_optional_statement(source)?.map(Box::new),
            }
        }

        "while_statement" => FruStatement::While {
            condition: ast.get_child("condition")?.parse_expression(source)?.wrap_box(),
            body: ast.get_child("body")?.parse_statement(source)?.wrap_box(),
        },

        "return_statement" => FruStatement::Return {
            value: ast.get_child("value").parse_optional_expression(source)?.map(Box::new),
        },

        "break_statement" => FruStatement::Break,

        "continue_statement" => FruStatement::Continue,

        "operator_statement" => FruStatement::Operator {
            ident: ast.get_child("ident")?.text(source)?.to_ident(),
            commutative: ast.get_child("commutative").is_ok(),
            left_ident: ast.get_child("left_ident")?.text(source)?.to_ident(),
            left_type_ident: ast.get_child("left_type_ident")?.text(source)?.to_ident(),
            right_ident: ast.get_child("right_ident")?.text(source)?.to_ident(),
            right_type_ident: ast.get_child("right_type_ident")?.text(source)?.to_ident(),
            body: ast.get_child("body")?.parse_function_body(source)?.wrap_rc(),
        },

        "type_statement" => {
            let type_type = match ast.get_child("type_type")?.text(source)? {
                "struct" => Ok(TypeType::Struct),
                "class" => Ok(TypeType::Class),
                "data" => Ok(TypeType::Data),
                _ => Err(ParseError::InvalidAst {
                    position: ast.get_child("type_type")?.range(),
                    error: format!("Invalid type type: {}", ast.get_child("type_type")?.text(source)?),
                }),
            }?;


            let ident = ast.get_child("ident")?.text(source)?.to_ident();

            let mut fields = Vec::new();
            let mut static_fields = Vec::new();

            for field in ast.children_by_field_name("fields", &mut ast.walk()) {
                match field.parse_field(source)? {
                    AnyField::Normal(f) => fields.push(f),
                    AnyField::Static(f) => static_fields.push(f),
                }
            }

            let mut methods = Vec::new();
            let mut static_methods = Vec::new();
            let mut watches = Vec::new();

            for section in ast.children_by_field_name("sections", &mut ast.walk()) {
                match section.parse_section(source)? {
                    TypeSection::Impl(x) => methods.extend(x),
                    TypeSection::Static(x) => static_methods.extend(x),
                    TypeSection::Constraints(x) => watches.extend(x),
                }
            }

            FruStatement::Type {
                type_type,
                ident,
                fields,
                static_fields,
                watches,
                methods,
                static_methods,
            }
        }

        x => unimplemented!("Not a statement: {} {}", x, ast.text(source)?),
    };

    Ok(result_statement)
}

fn parse_expression(ast: Node, source: &[u8]) -> Result<FruExpression, ParseError> {
    let result_expression = match ast.grammar_name() {
        "nah_literal" => FruExpression::Literal { value: FruValue::Nah },

        "number_literal" => FruExpression::Literal {
            value: FruValue::Number(
                ast.text(source)?.parse().unwrap(),
            )
        },

        "bool_literal" => FruExpression::Literal {
            value: FruValue::Bool(
                ast.text(source)?.parse().unwrap(),
            )
        },

        "string_literal" => {
            let s = ast.text(source)?;
            match unescape(&s.replace("\\\n", "\n")) {
                Ok(s) => FruExpression::Literal { value: FruValue::String(s) },
                Err(e) => return Err(ParseError::InvalidAst {
                    position: ast.range(),
                    error: e.to_string(),
                })
            }
        }

        "variable" => FruExpression::Variable {
            ident: ast.get_child("ident")?.text(source)?.to_ident()
        },

        "function_expression" => FruExpression::Function {
            args: ast
                .children_by_field_name("args", &mut ast.walk())
                .map(|x| x.text(source).map(Identifier::new))
                .try_collect()?,
            body: ast.get_child("body")?.parse_function_body(source)?.wrap_rc(),
        },

        "block_expression" => FruExpression::Block {
            body: ast
                .children_by_field_name("body", &mut ast.walk())
                .map(|x| x.parse_statement(source))
                .try_collect()?,
            expr: ast.get_child("expr")?.parse_expression(source)?.wrap_box(),
        },

        "call_expression" => FruExpression::Call {
            what: ast.get_child("what")?.parse_expression(source)?.wrap_box(),
            args: ast.children_by_field_name("args", &mut ast.walk())
                     .map(|x| x.parse_expression(source))
                     .try_collect()?,
        },

        "curry_call_expression" => FruExpression::CurryCall {
            what: ast.get_child("what")?.parse_expression(source)?.wrap_box(),
            args: ast.children_by_field_name("args", &mut ast.walk())
                     .map(|x| x.parse_expression(source))
                     .try_collect()?,
        },

        "instantiation_expression" => FruExpression::Instantiation {
            what: ast.get_child("what")?.parse_expression(source)?.wrap_box(),
            args: ast.children_by_field_name("args", &mut ast.walk())
                     .map(|x| x.parse_expression(source))
                     .try_collect()?,
        },

        "field_access_expression" => FruExpression::FieldAccess {
            what: ast.get_child("what")?.parse_expression(source)?.wrap_box(),
            field: ast.get_child("field")?.text(source)?.to_ident(),
        },

        "binary_expression" => FruExpression::Binary {
            operator: ast.get_child("operator")?.text(source)?.to_ident(),
            left: ast.get_child("left")?.parse_expression(source)?.wrap_box(),
            right: ast.get_child("right")?.parse_expression(source)?.wrap_box(),
        },

        "if_expression" => FruExpression::If {
            condition: ast.get_child("condition")?.parse_expression(source)?.wrap_box(),
            then_body: ast.get_child("then_body")?.parse_expression(source)?.wrap_box(),
            else_body: ast.get_child("else_body")?.parse_expression(source)?.wrap_box(),
        },

        _ => unimplemented!(
            "Not an expression: {} {}",
            ast.grammar_name(),
            ast.text(source)?
        ),
    };

    Ok(result_expression)
}

fn parse_function_body(ast: Node, source: &[u8]) -> Result<FruStatement, ParseError> {
    Ok(match ast.grammar_name() {
        "block_statement" => ast.parse_statement(source)?,
        "block_expression" => FruStatement::Return {
            value: Some(ast.parse_expression(source)?.wrap_box()),
        },
        _ => unimplemented!("Not a function body: {}", ast.grammar_name()),
    })
}

fn parse_field(ast: Node, source: &[u8]) -> Result<AnyField, ParseError> {
    let is_public = ast.get_child("pub").is_ok();
    let is_static = ast.get_child("static").is_ok();
    let ident = ast.get_child("ident")?.text(source)?.to_ident();
    let type_ident = ast.get_child("type_ident")
                        .ok().map(|x| x.text(source).map(Identifier::new)).transpose()?;
    let value = ast.get_child("value").parse_optional_expression(source)?;

    if !is_static && value.is_some() {
        let f = ast.get_child("value")?;
        panic!("Non-static field {} at {}-{} cannot have an default value", ident,
               f.start_position(),
               f.end_position(),
        );
    }

    let res = FruField {
        is_public,
        ident,
        type_ident,
    };

    Ok(if is_static {
        AnyField::Static((res, value.map(Box::new)))
    } else {
        AnyField::Normal(res)
    })
}

fn parse_section(ast: Node, source: &[u8]) -> Result<TypeSection, ParseError> {
    Ok(match ast.grammar_name() {
        "type_impl_section" => {
            TypeSection::Impl(
                ast.children_by_field_name("methods", &mut ast.walk())
                   .map(|x| x.parse_method(source))
                   .try_collect()?
            )
        }
        "type_static_section" => {
            TypeSection::Static(
                ast.children_by_field_name("methods", &mut ast.walk())
                   .map(|x| x.parse_method(source))
                   .try_collect()?
            )
        }
        "type_constraints_section" => {
            TypeSection::Constraints(
                ast.children_by_field_name("watches", &mut ast.walk())
                   .map(|x| x.parse_watch(source))
                   .try_collect()?
            )
        }

        _ => unimplemented!("Not a section: {}", ast.grammar_name()),
    })
}

fn parse_method(ast: Node, source: &[u8]) -> Result<(Identifier, Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let ident = ast.get_child("ident")?.text(source)?.to_ident();
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| x.text(source).map(Identifier::new))
                  .try_collect()?;
    let body = ast.get_child("body")?.parse_function_body(source)?.wrap_rc();
    Ok((ident, args, body))
}

fn parse_watch(ast: Node, source: &[u8]) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| x.text(source).map(Identifier::new))
                  .try_collect()?;

    let body = ast.get_child("body")?.parse_statement(source)?.wrap_rc();

    Ok((args, body))
}