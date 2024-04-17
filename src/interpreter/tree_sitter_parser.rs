use std::collections::{BTreeSet, LinkedList};
use std::rc::Rc;
use tree_sitter::Parser;
use tree_sitter_frugurt;

use crate::interpreter::value::operator::calculate_precedence;
use crate::interpreter::{
    expression::FruExpression, identifier::Identifier, statement::FruStatement,
    value::fru_value::FruValue,
};

pub fn parse(data: String) -> Box<FruStatement> {
    let bytes = data.as_bytes();

    let mut parser = Parser::new();

    parser // Todo: load grammar one time
        .set_language(&tree_sitter_frugurt::language())
        .expect("Error loading Frugurt grammar");

    let tree = parser.parse(bytes, None).unwrap();

    let root = tree.root_node();

    Box::new(parse_statement(root, data.as_bytes()))
}

fn parse_statement(ast: tree_sitter::Node, source: &[u8]) -> FruStatement {
    match ast.grammar_name() {
        "source_file" => FruStatement::Block(
            ast.children_by_field_name("body", &mut ast.walk())
                .map(|x| parse_statement(x, source))
                .collect(),
        ),

        "block_statement" => FruStatement::Block(
            ast.children_by_field_name("body", &mut ast.walk())
                .map(|x| parse_statement(x, source))
                .collect(),
        ),

        "expression_statement" => FruStatement::Expression {
            value: Box::new(parse_expression(
                ast.child_by_field_name("value").unwrap(),
                source,
            )),
        },

        "let_statement" => FruStatement::Let {
            ident: Identifier::new(
                ast.child_by_field_name("ident")
                    .unwrap()
                    .utf8_text(source)
                    .unwrap(),
            ),
            value: Box::new(parse_expression(
                ast.child_by_field_name("value").unwrap(),
                source,
            )),
        },

        "set_statement" => FruStatement::Set {
            ident: Identifier::new(
                ast.child_by_field_name("ident")
                    .unwrap()
                    .utf8_text(source)
                    .unwrap(),
            ),
            value: Box::new(parse_expression(
                ast.child_by_field_name("value").unwrap(),
                source,
            )),
        },

        "set_field_statement" => FruStatement::SetField {
            target: Box::new(parse_expression(
                ast.child_by_field_name("target").unwrap(),
                source,
            )),
            field: Identifier::new(
                ast.child_by_field_name("field")
                    .unwrap()
                    .utf8_text(source)
                    .unwrap(),
            ),
            value: Box::new(parse_expression(
                ast.child_by_field_name("value").unwrap(),
                source,
            )),
        },

        "if_statement" => FruStatement::If {
            condition: Box::new(parse_expression(
                ast.child_by_field_name("condition").unwrap(),
                source,
            )),
            then_body: Box::new(parse_statement(
                ast.child_by_field_name("then_body").unwrap(),
                source,
            )),
            else_body: ast
                .child_by_field_name("else_body")
                .map(|x| Box::new(parse_statement(x, source))),
        },

        "while_statement" => FruStatement::While {
            cond: Box::new(parse_expression(
                ast.child_by_field_name("condition").unwrap(),
                source,
            )),
            body: Box::new(parse_statement(
                ast.child_by_field_name("body").unwrap(),
                source,
            )),
        },

        "return_statement" => FruStatement::Return {
            value: ast
                .child_by_field_name("value")
                .map_or(Box::new(FruExpression::Literal(FruValue::Nah)), |x| {
                    Box::new(parse_expression(x, source))
                }),
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
            body: Rc::new(parse_function_body(
                ast.child_by_field_name("body").unwrap(),
                source,
            )),
        },

        x => unimplemented!("Not a statement: {} {}", x, ast.utf8_text(source).unwrap()),
    }
}

fn parse_expression(ast: tree_sitter::Node, source: &[u8]) -> FruExpression {
    match ast.grammar_name() {
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
                .collect(),
            expr: Box::new(parse_expression(
                ast.child_by_field_name("expr").unwrap(),
                source,
            )),
        },

        "call_expression" => FruExpression::Call {
            what: Box::new(parse_expression(
                ast.child_by_field_name("what").unwrap(),
                source,
            )),
            args: {
                ast.children_by_field_name("args", &mut ast.walk())
                    .map(|x| parse_expression(x, source))
                    .collect()
            },
        },

        "curry_call_expression" => FruExpression::CurryCall {
            what: Box::new(parse_expression(
                ast.child_by_field_name("what").unwrap(),
                source,
            )),
            args: {
                ast.children_by_field_name("args", &mut ast.walk())
                    .map(|x| parse_expression(x, source))
                    .collect()
            },
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
                    list.push_back(Elem::Expr(parse_expression(
                        ast.named_child(i).unwrap(),
                        source,
                    )));
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
            body: Rc::new(parse_function_body(
                ast.child_by_field_name("body").unwrap(),
                source,
            )),
        },

        "instantiation_expression" => FruExpression::Instantiation {
            what: Box::new(parse_expression(
                ast.child_by_field_name("what").unwrap(),
                source,
            )),
            args: {
                ast.children_by_field_name("args", &mut ast.walk())
                    .map(|x| parse_expression(x, source))
                    .collect()
            },
        },

        "field_access_expression" => FruExpression::FieldAccess {
            what: Box::new(parse_expression(
                ast.child_by_field_name("what").unwrap(),
                source,
            )),
            field: Identifier::new(
                ast.child_by_field_name("field")
                    .unwrap()
                    .utf8_text(source)
                    .unwrap(),
            ),
        },

        "if_expression" => FruExpression::If {
            condition: Box::new(parse_expression(
                ast.child_by_field_name("condition").unwrap(),
                source,
            )),

            then_body: Box::new(parse_expression(
                ast.child_by_field_name("then_body").unwrap(),
                source,
            )),

            else_body: Box::new(parse_expression(
                ast.child_by_field_name("else_body").unwrap(),
                source,
            )),
        },

        _ => unimplemented!(
            "Not an expression: {} {}",
            ast.grammar_name(),
            ast.utf8_text(source).unwrap()
        ),
    }
}

fn parse_function_body(ast: tree_sitter::Node, source: &[u8]) -> FruStatement {
    match ast.grammar_name() {
        "block_statement" => parse_statement(ast, source),
        "block_expression" => FruStatement::Return {
            value: Box::new(parse_expression(ast, source)),
        },
        _ => unimplemented!("Not a function body: {}", ast.grammar_name()),
    }
}
