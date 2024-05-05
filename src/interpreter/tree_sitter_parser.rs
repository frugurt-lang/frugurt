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
    value::function::{ArgumentList, FormalParameters},
};

#[derive(Error, Debug)]
pub enum ParseError {
    // needed ast is not provided by tree-sitter-frugurt (in theory could never happen)
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

    // ast provided by tree-sitter-frugurt is invalid (in theory could never happen)
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

    // invalid ast combination, that can only be caught by this parser
    #[error("{} at {}:{}-{}:{}",
    .error,
    .position.start_point.row + 1,
    .position.start_point.column,
    .position.end_point.row + 1,
    .position.end_point.column)]
    Error {
        position: Range,
        error: String,
    },
}

enum TypeExtension {
    Impl(Vec<(bool, Identifier, FormalParameters, Rc<FruStatement>)>),
    Constraints(Vec<(Vec<Identifier>, Rc<FruStatement>)>),
}

enum AnyField {
    Normal(FruField),
    Static((FruField, Option<Box<FruExpression>>)),
}

#[derive(Clone, Copy)]
struct NodeWrapper<'a> {
    node: Node<'a>,
    source: &'a [u8],
}

impl<'a> NodeWrapper<'a> {
    pub fn new(node: Node<'a>, source: &'a [u8]) -> Self {
        Self { node, source }
    }

    pub fn grammar_name(&self) -> &str {
        self.node.grammar_name()
    }

    pub fn range(&self) -> Range {
        self.node.range()
    }

    pub fn text(&self) -> Result<&'a str, ParseError> {
        self.node.utf8_text(self.source).map_err(
            |x| ParseError::Utf8Error {
                position: self.node.range(),
                error: x,
            })
    }

    pub fn ident(self) -> Result<Identifier, ParseError> {
        self.text().map(Identifier::new)
    }

    pub fn get_child(&self, name: &str) -> Result<Self, ParseError> {
        match self.node.child_by_field_name(name) {
            Some(x) => Ok(Self::new(x, self.source)),

            None => Err(ParseError::MissingAst {
                position: self.node.range(),
                name: name.to_string(),
            }),
        }
    }

    pub fn get_child_text(&self, name: &str) -> Result<&str, ParseError> {
        self.get_child(name)?.text()
    }

    pub fn get_child_ident(self, name: &str) -> Result<Identifier, ParseError> {
        Ok(Identifier::new(self.get_child_text(name)?))
    }

    pub fn parse_child_statement(self, name: &str) -> Result<FruStatement, ParseError> {
        parse_statement(self.get_child(name)?)
    }

    pub fn parse_child_expression(self, name: &str) -> Result<FruExpression, ParseError> {
        parse_expression(self.get_child(name)?)
    }

    pub fn parse_child<T>(&self, name: &str, parser: impl Fn(Self) -> Result<T, ParseError>)
                          -> Result<T, ParseError> {
        parser(self.get_child(name)?)
    }

    pub fn parse_children<T>(self, name: &str, parser: impl Fn(Self) -> Result<T, ParseError>)
                             -> Result<Vec<T>, ParseError> {
        self.node.children_by_field_name(name, &mut self.node.walk())
            .map(|x| parser(Self::new(x, self.source)))
            .try_collect()
    }

    pub fn parse_optional_child<T>(self, name: &str, parser: impl Fn(Self) -> Result<T, ParseError>)
                                   -> Result<Option<T>, ParseError> {
        match self.node.child_by_field_name(name) {
            Some(x) => Ok(Some(parser(Self::new(x, self.source))?)),
            None => Ok(None),
        }
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

    parse_statement(NodeWrapper::new(root, source)).map(Box::new)
}

fn search_for_errors(ast: Node) -> ParseError {
    let mut cur = ast.walk();

    loop {
        if cur.node().is_error() || cur.node().is_missing() {
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

fn parse_statement(ast: NodeWrapper) -> Result<FruStatement, ParseError> {
    let result_statement = match ast.grammar_name() {
        "source_file" => FruStatement::Block {
            body: ast.parse_children("body", parse_statement)?
        },

        "block_statement" => FruStatement::Block {
            body: ast.parse_children("body", parse_statement)?
        },

        "expression_statement" => FruStatement::Expression {
            value: ast.parse_child_expression("value")?.wrap_box(),
        },

        "let_statement" => FruStatement::Let {
            ident: ast.get_child_ident("ident")?,
            value: ast.parse_child_expression("value")?.wrap_box(),
        },

        "set_statement" => FruStatement::Set {
            ident: ast.get_child_ident("ident")?,
            value: ast.parse_child_expression("value")?.wrap_box(),
        },

        "set_field_statement" => {
            FruStatement::SetField {
                what: ast.parse_child_expression("what")?.wrap_box(),
                field: ast.get_child_ident("field")?,
                value: ast.parse_child_expression("value")?.wrap_box(),
            }
        }

        "if_statement" => {
            FruStatement::If {
                condition: ast.parse_child_expression("condition")?.wrap_box(),
                then_body: ast.parse_child_statement("then_body")?.wrap_box(),
                else_body: ast.parse_optional_child("else_body", parse_statement)?.map(Box::new),
            }
        }

        "while_statement" => FruStatement::While {
            condition: ast.parse_child_expression("condition")?.wrap_box(),
            body: ast.parse_child_statement("body")?.wrap_box(),
        },

        "return_statement" => FruStatement::Return {
            value: ast.parse_optional_child("value", parse_expression)?.map(Box::new),
        },

        "break_statement" => FruStatement::Break,

        "continue_statement" => FruStatement::Continue,

        "operator_statement" => {
            let commutative = ast.get_child("commutative").is_ok();

            let left_type_ident = ast.get_child_ident("left_type_ident")?;
            let right_type_ident = ast.get_child_ident("right_type_ident")?;

            if commutative && left_type_ident == right_type_ident {
                return Err(ParseError::Error {
                    position: ast.get_child("commutative")?.range(),
                    error: format!(
                        "commutative operators must have different types, but {} was used twice",
                        left_type_ident
                    ),
                });
            }

            FruStatement::Operator {
                ident: ast.get_child_ident("ident")?,
                commutative,
                left_ident: ast.get_child_ident("left_ident")?,
                left_type_ident,
                right_ident: ast.get_child_ident("right_ident")?,
                right_type_ident,
                body: ast.parse_child("body", parse_function_body)?.wrap_rc(),
            }
        }

        "type_statement" => {
            let type_type = match ast.get_child_text("type_type")? {
                "struct" => TypeType::Struct,
                "class" => TypeType::Class,
                "data" => TypeType::Data,
                _ => return Err(ParseError::InvalidAst {
                    position: ast.get_child("type_type")?.range(),
                    error: format!("Invalid type type: {}", ast.get_child_text("type_type")?),
                }),
            };

            let ident = ast.get_child_ident("ident")?;

            let mut fields = Vec::new();
            let mut static_fields = Vec::new();

            for field in ast.parse_children("fields", parse_field)? {
                match field {
                    AnyField::Normal(f) => fields.push(f),
                    AnyField::Static(f) => static_fields.push(f),
                }
            }

            let mut methods = Vec::new();

            let mut watches = Vec::new();

            for extension in ast.parse_children("extensions", parse_extension)? {
                match extension {
                    TypeExtension::Impl(xs) => {
                        methods.extend(xs);
                    }
                    TypeExtension::Constraints(x) => watches.extend(x),
                }
            }

            FruStatement::Type {
                type_type,
                ident,
                fields,
                static_fields,
                watches,
                methods,
            }
        }

        unexpected => return Err(ParseError::InvalidAst {
            position: ast.range(),
            error: format!("Not a statement: {}", unexpected),
        })
    };

    Ok(result_statement)
}

fn parse_expression(ast: NodeWrapper) -> Result<FruExpression, ParseError> {
    let result_expression = match ast.grammar_name() {
        "nah_literal" => FruExpression::Literal {
            value: FruValue::Nah
        },

        "number_literal" => FruExpression::Literal {
            value: FruValue::Number(
                ast.text()?.parse().unwrap(),
            )
        },

        "bool_literal" => FruExpression::Literal {
            value: FruValue::Bool(
                ast.text()?.parse().unwrap(),
            )
        },

        "string_literal" => {
            match unescape(&ast.text()?.replace("\\\n", "\n")) {
                Ok(s) => FruExpression::Literal {
                    value: FruValue::String(s)
                },

                Err(e) => return Err(ParseError::InvalidAst {
                    position: ast.range(),
                    error: e.to_string(),
                })
            }
        }

        "variable" => FruExpression::Variable {
            ident: ast.get_child_ident("ident")?
        },

        "function_expression" => FruExpression::Function {
            args: ast.parse_child("parameters", parse_formal_parameters)?,
            body: ast.parse_child("body", parse_function_body)?.wrap_rc(),
        },

        "block_expression" => FruExpression::Block {
            body: ast.parse_children("body", parse_statement)?,
            expr: ast.parse_child_expression("expr")?.wrap_box(),
        },

        "call_expression" => FruExpression::Call {
            what: ast.parse_child_expression("what")?.wrap_box(),
            args: ast.parse_child("args", parse_argument_list)?,
        },

        "curry_call_expression" => FruExpression::CurryCall {
            what: ast.parse_child_expression("what")?.wrap_box(),
            args: ast.parse_child("args", parse_argument_list)?,
        },

        "instantiation_expression" => FruExpression::Instantiation {
            what: ast.parse_child_expression("what")?.wrap_box(),
            args: ast.parse_child("args", parse_argument_list_instantiation)?,
        },

        "field_access_expression" => FruExpression::FieldAccess {
            what: ast.parse_child_expression("what")?.wrap_box(),
            field: ast.get_child_ident("field")?,
        },

        "binary_expression" => FruExpression::Binary {
            operator: ast.get_child_ident("operator")?,
            left: ast.parse_child_expression("left")?.wrap_box(),
            right: ast.parse_child_expression("right")?.wrap_box(),
        },

        "if_expression" => FruExpression::If {
            condition: ast.parse_child_expression("condition")?.wrap_box(),
            then_body: ast.parse_child_expression("then_body")?.wrap_box(),
            else_body: ast.parse_child_expression("else_body")?.wrap_box(),
        },

        unexpected => return Err(ParseError::InvalidAst {
            position: ast.range(),
            error: format!("Not an expression: {}", unexpected),
        })
    };

    Ok(result_expression)
}

fn parse_function_body(ast: NodeWrapper) -> Result<FruStatement, ParseError> {
    Ok(match ast.grammar_name() {
        "block_statement" => parse_statement(ast)?,
        "block_expression" => FruStatement::Return {
            value: Some(parse_expression(ast)?.wrap_box()),
        },

        unexpected => return Err(ParseError::InvalidAst {
            position: ast.range(),
            error: format!("Not a function body: {}", unexpected),
        })
    })
}

fn parse_field(ast: NodeWrapper) -> Result<AnyField, ParseError> {
    let is_public = ast.get_child("pub").is_ok();
    let is_static = ast.get_child("static").is_ok();
    let ident = ast.get_child_ident("ident")?;
    let type_ident = ast.parse_optional_child("type_ident", NodeWrapper::ident)?;

    let value = ast.parse_optional_child("value", parse_expression)?;

    if !is_static && value.is_some() {
        return Err(ParseError::Error {
            position: ast.get_child("value")?.range(),
            error: "Non-static field cannot have an initial value".to_string(),
        });
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

fn parse_extension(ast: NodeWrapper) -> Result<TypeExtension, ParseError> {
    Ok(match ast.grammar_name() {
        "type_impl_extension" => {
            TypeExtension::Impl(
                ast.parse_children("methods", parse_method)?
            )
        }
        "type_constraints_extension" => {
            TypeExtension::Constraints(
                ast.parse_children("watches", parse_watch)?
            )
        }

        unexpected => return Err(ParseError::InvalidAst {
            position: ast.range(),
            error: format!("Not a type extension: {}", unexpected),
        })
    })
}

fn parse_method(ast: NodeWrapper) -> Result<(bool, Identifier, FormalParameters, Rc<FruStatement>), ParseError> {
    let is_static = ast.get_child("static").is_ok();

    let ident = ast.get_child_ident("ident")?;

    let args = ast.parse_child("parameters", parse_formal_parameters)?;

    let body = ast.parse_child("body", parse_function_body)?.wrap_rc();

    Ok((is_static, ident, args, body))
}

fn parse_watch(ast: NodeWrapper) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let args = ast.parse_children("args", NodeWrapper::ident)?;

    let body = ast.parse_child_statement("body")?.wrap_rc();

    Ok((args, body))
}

fn parse_formal_parameters(ast: NodeWrapper) -> Result<FormalParameters, ParseError> {
    let args = ast.parse_children("args", parse_formal_parameter)?;

    let mut was_default = false;
    let mut minimum_args = 0;

    for i in 0..args.len() {
        if args[i].1.is_some() {
            was_default = true;
        } else if was_default {
            return Err(ParseError::Error {
                position: ast.parse_children("args", Ok)?[i].range(),
                error: "Positional parameters should be before default parameters".to_string(),
            });
        } else {
            minimum_args += 1;
        }
    }

    Ok(FormalParameters {
        args,
        minimum_args,
    })
}

fn parse_formal_parameter(x: NodeWrapper) -> Result<(Identifier, Option<FruExpression>), ParseError> {
    match x.grammar_name() {
        "positional_parameter" => {
            Ok((
                x.get_child_ident("ident")?,
                None
            ))
        }

        "default_parameter" => {
            Ok((
                x.get_child_ident("ident")?,
                Some(x.parse_child_expression("value")?),
            ))
        }
        unexpected => Err(ParseError::InvalidAst {
            position: x.range(),
            error: format!("Not a formal parameter: {}", unexpected),
        })
    }
}

fn parse_argument_list(ast: NodeWrapper) -> Result<ArgumentList, ParseError> {
    let args = ast.parse_children("args", parse_argument_item)?;

    let mut was_named = false;

    for i in 0..args.len() {
        if args[i].0.is_some() {
            was_named = true;
        } else if was_named {
            return Err(ParseError::Error {
                position: ast.parse_children("args", Ok)?[i].range(),
                error: "Positional arguments should be before named arguments".to_string(),
            });
        }
    }

    Ok(ArgumentList { args })
}

fn parse_argument_list_instantiation(ast: NodeWrapper) -> Result<ArgumentList, ParseError> {
    let args = parse_argument_list(ast)?;

    if args.args.len() == 0 {
        return Ok(args);
    }

    let named = args.args[0].0.is_some();

    for (i, _) in &args.args {
        if i.is_some() != named {
            return Err(ParseError::Error {
                position: ast.range(),
                error: "All arguments must be either named or not named at the same time".to_string(),
            });
        }
    }

    Ok(args)
}

fn parse_argument_item(ast: NodeWrapper) -> Result<(Option<Identifier>, FruExpression), ParseError> {
    match ast.grammar_name() {
        "positional_argument" => {
            Ok((
                None,
                ast.parse_child_expression("value")?
            ))
        }

        "named_argument" => {
            Ok((
                Some(ast.get_child_ident("ident")?),
                ast.parse_child_expression("value")?
            ))
        }

        unexpected => return Err(ParseError::InvalidAst {
            position: ast.range(),
            error: format!("Not an argument: {}", unexpected),
        })
    }
}
