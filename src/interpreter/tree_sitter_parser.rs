use std::{
    boxed::Box,
    collections::{BTreeSet, LinkedList},
    rc::Rc,
    str::Utf8Error,
};

use thiserror::Error;
use tree_sitter::{Node, Parser, Range};
use tree_sitter_frugurt;

use crate::interpreter::{
    expression::FruExpression,
    identifier::Identifier,
    statement::FruStatement,
    value::fru_type::FruField,
    value::fru_type::TypeType,
    value::fru_value::FruValue,
    value::operator::calculate_precedence,
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
    #[error("ast is not valid at {}:{}-{}:{}",
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
}

pub fn parse(data: String) -> Result<Box<FruStatement>, ParseError> {
    let source = data.as_bytes();

    let mut parser = Parser::new();

    parser // Todo: load grammar one time
        .set_language(&tree_sitter_frugurt::language())
        .expect("Error loading Frugurt grammar");

    let tree = parser.parse(source, None).unwrap();

    let root = tree.root_node();

    if root.has_error() {
        search_for_errors(root)?;
    }

    parse_statement(root, source).map(Box::new)
}

fn search_for_errors(ast: Node) -> Result<(), ParseError> {
    let mut cur = ast.walk();

    loop {
        if cur.node().is_error() {
            return Err(ParseError::ParsingError {
                position: cur.node().range(),
            });
        }

        if cur.goto_first_child() {
            continue;
        }

        if cur.goto_next_sibling() {
            continue;
        }

        if cur.goto_parent() {
            cur.goto_next_sibling();
        } else {
            break;
        }
    }

    Ok(())
}


fn option_statement(x: Result<Node, ParseError>, source: &[u8]) -> Result<Option<FruStatement>, ParseError> {
    Ok(match x {
        Ok(x) => Some(parse_statement(x, source)?),
        Err(_) => None,
    })
}

fn option_expression(x: Result<Node, ParseError>, source: &[u8]) -> Result<Option<FruExpression>, ParseError> {
    Ok(match x {
        Ok(x) => Some(parse_expression(x, source)?),
        Err(_) => None,
    })
}


fn parse_statement(ast: Node, source: &[u8]) -> Result<FruStatement, ParseError> {
    let result_statement = match ast.grammar_name() {
        "source_file" => FruStatement::Block(
            ast.children_by_field_name("body", &mut ast.walk())
               .map(|x| parse_statement(x, source))
               .try_collect()?,
        ),

        "block_statement" => FruStatement::Block(
            ast.children_by_field_name("body", &mut ast.walk())
               .map(|x| parse_statement(x, source))
               .try_collect()?,
        ),

        "expression_statement" => FruStatement::Expression {
            value: Box::new(
                parse_expression(
                    ast.get_child("value")?,
                    source,
                )?
            ),
        },

        "let_statement" => FruStatement::Let {
            ident: Identifier::new(
                ast.get_child("ident")?
                    .text(source)?,
            ),
            value: Box::new(
                parse_expression(
                    ast.get_child("value")?,
                    source,
                )?
            ),
        },

        "set_statement" => FruStatement::Set {
            ident: Identifier::new(
                ast.get_child("ident")
                    ?
                    .text(source)?,
            ),
            value: Box::new(
                parse_expression(
                    ast.get_child("value")?,
                    source,
                )?
            ),
        },

        "set_field_statement" => {
            let what = parse_expression(
                ast.get_child("what")?,
                source,
            )?;

            let value = parse_expression(
                ast.get_child("value")?,
                source,
            )?;

            match what {
                FruExpression::FieldAccess { what, field } => FruStatement::SetField {
                    target: what,
                    field,
                    value: Box::new(value),
                },

                _ => panic!("set_field_statement: what is not a field access {:?}", what),
            }
        }

        "if_statement" => {
            let condition = parse_expression(
                ast.get_child("condition")?,
                source,
            )?;
            let then_body = parse_statement(
                ast.get_child("then_body")?,
                source,
            )?;
            let else_body = option_statement(
                ast.get_child("else_body"),
                source,
            )?;

            FruStatement::If {
                condition: Box::new(condition),
                then_body: Box::new(then_body),
                else_body: else_body.map(Box::new),
            }
        }

        "while_statement" => FruStatement::While {
            cond: Box::new(
                parse_expression(
                    ast.get_child("condition")?,
                    source,
                )?
            ),
            body: Box::new(
                parse_statement(
                    ast.get_child("body")?,
                    source,
                )?
            ),
        },

        "return_statement" => FruStatement::Return {
            value: option_expression(
                ast.get_child("value"),
                source,
            )?.map(Box::new),
        },

        "break_statement" => FruStatement::Break,

        "continue_statement" => FruStatement::Continue,

        "operator_statement" => FruStatement::Operator {
            ident: Identifier::new(
                ast.get_child("ident")
                    ?
                    .text(source)?,
            ),

            commutative: ast.get_child("commutative").is_ok(),
            left_ident: Identifier::new(
                ast.get_child("left_ident")
                    ?
                    .text(source)?,
            ),
            left_type_ident: Identifier::new(
                ast.get_child("left_type_ident")
                    ?
                    .text(source)?,
            ),
            right_ident: Identifier::new(
                ast.get_child("right_ident")
                    ?
                    .text(source)?,
            ),
            right_type_ident: Identifier::new(
                ast.get_child("right_type_ident")
                    ?
                    .text(source)?,
            ),
            body: Rc::new(
                parse_function_body(
                    ast.get_child("body")?,
                    source,
                )?
            ),
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


            let ident = Identifier::new(ast.get_child("ident")?.text(source)?);

            let mut fields = Vec::new();
            let mut static_fields = Vec::new();

            for field in ast.children_by_field_name("fields", &mut ast.walk())
                            .map(|x| parse_field(x, source)) {
                match field? {
                    AnyField::Normal(f) => fields.push(f),
                    AnyField::Static(f) => static_fields.push(f),
                }
            }

            let mut methods = Vec::new();
            let mut static_methods = Vec::new();
            let mut watches = Vec::new();

            for section in ast.children_by_field_name("sections", &mut ast.walk()) {
                match parse_section(section, source)? {
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
        "nah_literal" => FruExpression::Literal(FruValue::Nah),

        "number_literal" => FruExpression::Literal(FruValue::Number(
            ast.text(source)?.parse().unwrap(),
        )),

        "bool_literal" => FruExpression::Literal(FruValue::Bool(
            ast.text(source)?.parse().unwrap(),
        )),

        "string_literal" => {
            let s = ast.text(source)?;
            FruExpression::Literal(FruValue::String(s[1..s.len() - 1].to_string()))
        }

        "variable" => FruExpression::Variable(Identifier::new(
            ast.child(0).unwrap().text(source)?, // FIXME: unwrap
        )),

        "block_expression" => FruExpression::Block {
            body: ast
                .children_by_field_name("body", &mut ast.walk())
                .map(|x| parse_statement(x, source))
                .try_collect()?,
            expr: Box::new(
                parse_expression(
                    ast.get_child("expr")?,
                    source,
                )?
            ),
        },

        "call_expression" => FruExpression::Call {
            what: Box::new(
                parse_expression(
                    ast.get_child("what")?,
                    source,
                )?
            ),
            args: ast.children_by_field_name("args", &mut ast.walk())
                     .map(|x| parse_expression(x, source))
                     .try_collect()?,
        },

        "curry_call_expression" => FruExpression::CurryCall {
            what: Box::new(
                parse_expression(
                    ast.get_child("what")?,
                    source,
                )?
            ),
            args: ast.children_by_field_name("args", &mut ast.walk())
                     .map(|x| parse_expression(x, source))
                     .try_collect()?,
        },

        "binaries_expression" => {
            enum Elem {
                Expr(FruExpression),
                BinOp { ident: Identifier, precedence: i32 },
            }

            let mut list = LinkedList::new();

            let mut all_precedences = BTreeSet::new();

            for i in 0..ast.named_child_count() {
                if i % 2 == 0 {
                    list.push_back(Elem::Expr(
                        parse_expression(
                            ast.named_child(i).unwrap(),
                            source,
                        )?
                    ));
                } else {
                    let op = ast.named_child(i).unwrap().text(source)?;
                    let precedence = calculate_precedence(op);

                    all_precedences.insert(precedence);
                    list.push_back(Elem::BinOp {
                        ident: Identifier::new(op),
                        precedence,
                    });
                }
            }

            for target_precedence in all_precedences {
                let mut cursor = list.cursor_front_mut();
                cursor.move_next();

                loop {
                    let ident = match cursor.current() {
                        None => break,
                        Some(Elem::BinOp { precedence, ident })
                        if *precedence == target_precedence =>
                            {
                                *ident
                            }
                        _ => {
                            cursor.move_next();
                            continue;
                        }
                    };

                    cursor.move_prev();

                    let left = cursor.remove_current().unwrap();
                    cursor.remove_current();
                    let right = cursor.remove_current().unwrap();

                    cursor.insert_before(Elem::Expr(FruExpression::Binary {
                        operator: ident,
                        left: Box::new(match left {
                            Elem::Expr(expr) => expr,
                            _ => panic!(),
                        }),

                        right: Box::new(match right {
                            Elem::Expr(expr) => expr,
                            _ => panic!(),
                        }),
                    }));
                }
            }

            match list.pop_front().unwrap() {
                Elem::Expr(expr) => expr,
                _ => panic!(),
            }
        }

        "function_expression" => FruExpression::Function {
            args: ast
                .children_by_field_name("args", &mut ast.walk())
                .map(|x| x.text(source).map(Identifier::new))
                .try_collect()?,
            body: Rc::new(
                parse_function_body(
                    ast.get_child("body")?,
                    source,
                )?
            ),
        },

        "instantiation_expression" => FruExpression::Instantiation {
            what: Box::new(
                parse_expression(
                    ast.get_child("what")?,
                    source,
                )?),
            args: {
                ast.children_by_field_name("args", &mut ast.walk())
                   .map(|x| parse_expression(x, source))
                   .try_collect()?
            },
        },

        "field_access_expression" => FruExpression::FieldAccess {
            what: Box::new(
                parse_expression(
                    ast.get_child("what")?,
                    source,
                )?
            ),
            field: Identifier::new(
                ast.get_child("field")
                    ?
                    .text(source)?,
            ),
        },

        "if_expression" => FruExpression::If {
            condition: Box::new(
                parse_expression(
                    ast.get_child("condition")?,
                    source,
                )?
            ),

            then_body: Box::new(
                parse_expression(
                    ast.get_child("then_body")?,
                    source,
                )?
            ),

            else_body: Box::new(
                parse_expression(
                    ast.get_child("else_body")?,
                    source,
                )?
            ),
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
        "block_statement" => parse_statement(ast, source)?,
        "block_expression" => FruStatement::Return {
            value: Some(Box::new(parse_expression(ast, source)?)),
        },
        _ => unimplemented!("Not a function body: {}", ast.grammar_name()),
    })
}

fn parse_field(ast: Node, source: &[u8]) -> Result<AnyField, ParseError> {
    let is_public = ast.get_child("pub").is_ok();
    let is_static = ast.get_child("static").is_ok();
    let ident = Identifier::new(ast.get_child("ident")?
        .text(source)?);
    let type_ident = ast.get_child("type_ident")
                        .ok().map(|x| x.text(source).map(Identifier::new)).transpose()?;
    let value = option_expression(
        ast.get_child("value"),
        source)?;

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
                   .map(|x| parse_method(x, source)).try_collect()?
            )
        }
        "type_static_section" => {
            TypeSection::Static(
                ast.children_by_field_name("methods", &mut ast.walk())
                   .map(|x| parse_method(x, source)).try_collect()?
            )
        }
        "type_constraints_section" => {
            TypeSection::Constraints(
                ast.children_by_field_name("watches", &mut ast.walk())
                   .map(|x| parse_watch(x, source)).try_collect()?
            )
        }

        _ => unimplemented!("Not a section: {}", ast.grammar_name()),
    })
}

fn parse_method(ast: Node, source: &[u8]) -> Result<(Identifier, Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let ident = Identifier::new(ast.get_child("ident")?.text(source)?);
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| x.text(source).map(Identifier::new)).try_collect()?;
    let body = Rc::new(parse_function_body(ast.get_child("body")?, source)?);
    Ok((ident, args, body))
}

fn parse_watch(ast: Node, source: &[u8]) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| x.text(source).map(Identifier::new)).try_collect()?;

    let body = Rc::new(parse_statement(ast.get_child("body")?, source)?);

    Ok((args, body))
}