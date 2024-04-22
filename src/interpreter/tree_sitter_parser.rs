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
    value::fru_value::FruValue,
    value::operator::calculate_precedence,
};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Bug in ast provided by tree-sitter-frugurt")]
    MissingAst {
        position: Range,
        name: String,
    },

    #[error("Utf8 decoding error")]
    Utf8Decode {
        position: Range,
        error: Utf8Error,
    },
}

fn remap_statement(x: Option<Node>, source: &[u8]) -> Result<Option<FruStatement>, ParseError> {
    match x {
        Some(x) => Ok(Some(parse_statement(x, source)?)),
        None => Ok(None),
    }
}

fn remap_expression(x: Option<Node>, source: &[u8]) -> Result<Option<FruExpression>, ParseError> {
    match x {
        Some(x) => Ok(Some(parse_expression(x, source)?)),
        None => Ok(None),
    }
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

pub fn parse(data: String) -> Result<Box<FruStatement>, ParseError> {
    let bytes = data.as_bytes();

    let mut parser = Parser::new();

    parser // Todo: load grammar one time
        .set_language(&tree_sitter_frugurt::language())
        .expect("Error loading Frugurt grammar");

    let tree = parser.parse(bytes, None).unwrap();

    let root = tree.root_node();

    dbg!(parse_statement(root, data.as_bytes())).map(Box::new)
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
                    ast.child_by_field_name("value")
                       .ok_or_else(|| ParseError::MissingAst {
                           position: ast.range(),
                           name: "value".to_string(),
                       })?,
                    source,
                )?
            ),
        },

        "let_statement" => FruStatement::Let {
            ident: Identifier::new(
                ast.child_by_field_name("ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            value: Box::new(
                parse_expression(
                    ast.child_by_field_name("value").unwrap(),
                    source,
                )?
            ),
        },

        "set_statement" => FruStatement::Set {
            ident: Identifier::new(
                ast.child_by_field_name("ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            value: Box::new(
                parse_expression(
                    ast.child_by_field_name("value").unwrap(),
                    source,
                )?
            ),
        },

        "set_field_statement" => {
            let what = parse_expression(
                ast.child_by_field_name("what").unwrap(),
                source,
            )?;

            let value = parse_expression(
                ast.child_by_field_name("value").unwrap(),
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
                ast.child_by_field_name("condition").unwrap(),
                source,
            )?;
            let then_body = parse_statement(
                ast.child_by_field_name("then_body").unwrap(),
                source,
            )?;
            let else_body = remap_statement(
                ast.child_by_field_name("else_body"),
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
                    ast.child_by_field_name("condition").unwrap(),
                    source,
                )?
            ),
            body: Box::new(
                parse_statement(
                    ast.child_by_field_name("body").unwrap(),
                    source,
                )?
            ),
        },

        "return_statement" => FruStatement::Return {
            value: remap_expression(
                ast.child_by_field_name("value"),
                source,
            )?.map(Box::new),
        },

        "break_statement" => FruStatement::Break,

        "continue_statement" => FruStatement::Continue,

        "operator_statement" => FruStatement::Operator {
            ident: Identifier::new(
                ast.child_by_field_name("ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),

            commutative: ast.child_by_field_name("commutative").is_some(),
            left_ident: Identifier::new(
                ast.child_by_field_name("left_ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            left_type_ident: Identifier::new(
                ast.child_by_field_name("left_type_ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            right_ident: Identifier::new(
                ast.child_by_field_name("right_ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            right_type_ident: Identifier::new(
                ast.child_by_field_name("right_type_ident")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
            body: Rc::new(
                parse_function_body(
                    ast.child_by_field_name("body").unwrap(),
                    source,
                )?
            ),
        },

        "type_statement" => {
            let type_type = ast.child_by_field_name("type_type").unwrap()
                               .utf8_text(source).unwrap().try_into().unwrap();
            let ident = Identifier::new(ast.child_by_field_name("ident").unwrap().utf8_text(source).unwrap());

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

        x => unimplemented!("Not a statement: {} {}", x, ast.utf8_text(source).unwrap()),
    };

    Ok(result_statement)
}

fn parse_expression(ast: Node, source: &[u8]) -> Result<FruExpression, ParseError> {
    let result_expression = match ast.grammar_name() {
        "nah_literal" => FruExpression::Literal(FruValue::Nah),

        "number_literal" => FruExpression::Literal(FruValue::Number(
            ast.utf8_text(source).unwrap().parse().unwrap(),
        )),

        "bool_literal" => FruExpression::Literal(FruValue::Bool(
            ast.utf8_text(source).unwrap().parse().unwrap(),
        )),

        "string_literal" => {
            let s = ast.utf8_text(source).unwrap();
            FruExpression::Literal(FruValue::String(s[1..s.len() - 1].to_string()))
        }

        "variable" => FruExpression::Variable(Identifier::new(
            ast.child(0).unwrap().utf8_text(source).unwrap(),
        )),

        "block_expression" => FruExpression::Block {
            body: ast
                .children_by_field_name("body", &mut ast.walk())
                .map(|x| parse_statement(x, source))
                .try_collect()?,
            expr: Box::new(
                parse_expression(
                    ast.child_by_field_name("expr").unwrap(),
                    source,
                )?
            ),
        },

        "call_expression" => FruExpression::Call {
            what: Box::new(
                parse_expression(
                    ast.child_by_field_name("what").unwrap(),
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
                    ast.child_by_field_name("what").unwrap(),
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
                    let op = ast.named_child(i).unwrap().utf8_text(source).unwrap();
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
                .map(|x| Identifier::new(x.utf8_text(source).unwrap()))
                .collect(),
            body: Rc::new(
                parse_function_body(
                    ast.child_by_field_name("body").unwrap(),
                    source,
                )?
            ),
        },

        "instantiation_expression" => FruExpression::Instantiation {
            what: Box::new(
                parse_expression(
                    ast.child_by_field_name("what").unwrap(),
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
                    ast.child_by_field_name("what").unwrap(),
                    source,
                )?
            ),
            field: Identifier::new(
                ast.child_by_field_name("field")
                   .unwrap()
                   .utf8_text(source)
                   .unwrap(),
            ),
        },

        "if_expression" => FruExpression::If {
            condition: Box::new(
                parse_expression(
                    ast.child_by_field_name("condition").unwrap(),
                    source,
                )?
            ),

            then_body: Box::new(
                parse_expression(
                    ast.child_by_field_name("then_body").unwrap(),
                    source,
                )?
            ),

            else_body: Box::new(
                parse_expression(
                    ast.child_by_field_name("else_body").unwrap(),
                    source,
                )?
            ),
        },

        _ => unimplemented!(
            "Not an expression: {} {}",
            ast.grammar_name(),
            ast.utf8_text(source).unwrap()
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
    let is_public = ast.child_by_field_name("pub").is_some();
    let is_static = ast.child_by_field_name("static").is_some();
    let ident = Identifier::new(ast.child_by_field_name("ident").unwrap()
                                   .utf8_text(source).unwrap());
    let type_ident = ast.child_by_field_name("type_ident")
                        .map(|x| Identifier::new(x.utf8_text(source).unwrap()));
    let value = remap_expression(
        ast.child_by_field_name("value"),
        source)?;

    if !is_static && value.is_some() {
        let f = ast.child_by_field_name("value").unwrap();
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
    let ident = Identifier::new(ast.child_by_field_name("ident").unwrap().utf8_text(source).unwrap());
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| Identifier::new(x.utf8_text(source).unwrap())).collect();
    let body = Rc::new(parse_function_body(ast.child_by_field_name("body").unwrap(), source)?);
    Ok((ident, args, body))
}

fn parse_watch(ast: Node, source: &[u8]) -> Result<(Vec<Identifier>, Rc<FruStatement>), ParseError> {
    let args = ast.children_by_field_name("args", &mut ast.walk())
                  .map(|x| Identifier::new(x.utf8_text(source).unwrap())).collect();

    let body = Rc::new(parse_statement(ast.child_by_field_name("body").unwrap(), source)?);

    Ok((args, body))
}