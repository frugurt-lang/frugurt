use serde_json::Value;

use std::{
    collections::{BTreeSet, LinkedList},
    rc::Rc,
};

use crate::interpreter::{
    expression::FruExpression,
    identifier::Identifier,
    statement::{FruStatement, TypeType},
    value::fru_type::FruField,
    value::fru_value::FruValue,
    value::operator::calculate_precedence,
};

pub fn parse(data: Value) -> Box<FruStatement> {
    Box::new(convert_to_stmt(&data))
}

macro_rules! json_in_invalid {
    () => {
        panic!("json is invalid")
    };
    ($message:tt, $($args:tt)+) => {
        panic!("json is invalid: {}", format!($message, $($args)+))
    }
}

fn new_id(x: &Value) -> Identifier {
    Identifier::new(x.as_str().unwrap())
}

fn convert_to_expr(ast: &Value) -> FruExpression {
    let t = ast["node"].as_str().unwrap();

    match t {
        "literal" => match ast["value"] {
            Value::Number(ref n) => FruExpression::Literal(FruValue::Number(
                n.as_f64()
                    .unwrap_or_else(|| json_in_invalid!("{} is not a number", n)),
            )),

            Value::Bool(ref v) => FruExpression::Literal(FruValue::Bool(*v)),

            Value::String(ref s) => FruExpression::Literal(FruValue::String(s.clone())),

            Value::Null => FruExpression::Literal(FruValue::Nah),

            _ => json_in_invalid!("{} is not a literal", ast["value"]),
        },

        "variable" => FruExpression::Variable(new_id(&ast["ident"])),

        "block" => {
            let body = ast["body"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_stmt)
                .collect();

            let expr = convert_to_expr(&ast["expr"]);

            FruExpression::Block {
                body,
                expr: Box::new(expr),
            }
        }

        "call" => {
            let what = convert_to_expr(&ast["what"]);
            let args = ast["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_expr)
                .collect();

            FruExpression::Call {
                what: Box::new(what),
                args,
            }
        }

        "curry" => {
            let what = convert_to_expr(&ast["what"]);
            let args = ast["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_expr)
                .collect();

            FruExpression::CurryCall {
                what: Box::new(what),
                args,
            }
        }

        "binaries" => {
            let first = convert_to_expr(&ast["first"]);
            let mut rest = convert_to_rest(&ast["rest"]);

            let all_precedences: BTreeSet<_> = rest.iter().map(|x| x.0 .0).collect();

            rest.push_front(((100, Identifier::for_none()), Box::new(first)));

            for precedence in all_precedences {
                let mut cursor = rest.cursor_front_mut();

                while let Some(next) = cursor.peek_next() {
                    if next.0 .0 != precedence {
                        cursor.move_next();
                        continue;
                    }

                    let next_val = next.clone();

                    let curr = cursor.current().unwrap();

                    *curr.1 = FruExpression::Binary {
                        operator: next_val.0 .1,
                        left: curr.1.clone(),
                        right: next_val.1.clone(),
                    };

                    cursor.move_next();
                    cursor.remove_current();
                    cursor.move_prev();
                }
            }

            *rest.front().unwrap().1.clone()
        }

        "function" => {
            let args = ast["args"].as_array().unwrap().iter().map(new_id).collect();
            let body = convert_to_stmt(&ast["body"]);

            FruExpression::Function {
                args,
                body: Rc::new(body),
            }
        }

        "instantiation" => {
            let what = convert_to_expr(&ast["what"]);
            let args = ast["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_expr)
                .collect();

            FruExpression::Instantiation {
                what: Box::new(what),
                args,
            }
        }

        "field_access" => {
            let what = convert_to_expr(&ast["what"]);
            let field = new_id(&ast["field"]);

            FruExpression::FieldAccess {
                what: Box::new(what),
                field,
            }
        }

        "if_expr" => {
            let condition = convert_to_expr(&ast["cond"]);
            let then_body = convert_to_expr(&ast["then"]);
            let else_body = convert_to_expr(&ast["else"]);

            FruExpression::If {
                condition: Box::new(condition),
                then_body: Rc::new(then_body),
                else_body: Rc::new(else_body),
            }
        }

        unknown => json_in_invalid!("{} is not an expression", unknown),
    }
}

fn convert_to_stmt(ast: &Value) -> FruStatement {
    let t = ast["node"].as_str().unwrap();

    match t {
        "block" => {
            let body = ast["body"].as_array().unwrap();

            FruStatement::Block(body.iter().map(convert_to_stmt).collect())
        }

        "nothing" => FruStatement::Nothing,

        "expression" => {
            let value = convert_to_expr(&ast["value"]);
            FruStatement::Expression {
                value: Box::new(value),
            }
        }

        "let" => {
            let ident = new_id(&ast["ident"]);
            let value = convert_to_expr(&ast["value"]);

            FruStatement::Let {
                ident,
                value: Box::new(value),
            }
        }

        "set" => {
            let ident = new_id(&ast["ident"]);
            let value = convert_to_expr(&ast["value"]);

            FruStatement::Set {
                ident,
                value: Box::new(value),
            }
        }

        "set_field" => {
            let target = convert_to_expr(&ast["target"]);
            let field = new_id(&ast["field"]);
            let value = convert_to_expr(&ast["value"]);

            FruStatement::SetField {
                target: Box::new(target),
                field,
                value: Box::new(value),
            }
        }

        "if" => {
            let cond = convert_to_expr(&ast["cond"]);
            let then = convert_to_stmt(&ast["then"]);
            let else_ = convert_to_stmt(&ast["else"]);

            FruStatement::If {
                cond: Box::new(cond),
                then_body: Box::new(then),
                else_body: Box::new(else_),
            }
        }

        "while" => {
            let cond = convert_to_expr(&ast["cond"]);
            let body = convert_to_stmt(&ast["body"]);

            FruStatement::While {
                cond: Box::new(cond),
                body: Box::new(body),
            }
        }

        "return" => {
            let value = convert_to_expr(&ast["value"]);

            FruStatement::Return {
                value: Box::new(value),
            }
        }

        "break" => FruStatement::Break,

        "continue" => FruStatement::Continue,

        "operator" => {
            let ident = new_id(&ast["ident"]);
            let commutative = ast["commutative"].as_bool().unwrap();
            let left_ident = new_id(&ast["left_ident"]);
            let left_type_ident = new_id(&ast["left_type_ident"]);
            let right_ident = new_id(&ast["right_ident"]);
            let right_type_ident = new_id(&ast["right_type_ident"]);
            let body = convert_to_stmt(&ast["body"]);

            FruStatement::OperatorDefinition {
                ident,
                commutative,
                left_ident,
                left_type_ident,
                right_ident,
                right_type_ident,
                body: Rc::new(body),
            }
        }

        "type" => {
            let type_type = match ast["type"].as_str().unwrap() {
                "struct" => TypeType::Struct,
                other => json_in_invalid!("only structs are supported now, not {}", other),
            };

            let ident = new_id(&ast["ident"]);

            let raw_fields = ast["fields"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_fru_field);

            let watches: Vec<(Vec<Identifier>, Rc<FruStatement>)> = ast["watches"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_raw_watch)
                .collect();

            let methods = ast["methods"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_fru_method)
                .collect();

            let static_methods = ast["static_methods"]
                .as_array()
                .unwrap()
                .iter()
                .map(convert_to_fru_method)
                .collect();

            let mut fields = vec![];

            let mut static_fields = vec![];

            for (field, is_static, value) in raw_fields {
                if !is_static {
                    fields.push(field);
                } else {
                    static_fields.push((field, value))
                }
            }

            FruStatement::TypeDeclaration {
                type_type,
                ident,
                fields,
                static_fields,
                watches,
                methods,
                static_methods,
            }
        }

        unknown => json_in_invalid!("{} is not an statement", unknown),
    }
}

fn convert_to_fru_field(ast: &Value) -> (FruField, bool, Option<Box<FruExpression>>) {
    let ident = new_id(&ast["ident"]);
    let is_public = ast["is_pub"].as_bool().unwrap();
    let is_static = ast["is_static"].as_bool().unwrap();
    let type_ident = ast["type_ident"].as_str().map(Identifier::new);
    let value = ast.get("value").map(|x| Box::new(convert_to_expr(x)));
    (
        FruField {
            is_public,
            ident,
            type_ident,
        },
        is_static,
        value,
    )
}

fn convert_to_raw_watch(ast: &Value) -> (Vec<Identifier>, Rc<FruStatement>) {
    let fields = ast["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(new_id)
        .collect();
    let body = convert_to_stmt(&ast["body"]);
    (fields, Rc::new(body))
}

fn convert_to_fru_method(ast: &Value) -> (Identifier, Vec<Identifier>, Rc<FruStatement>) {
    let ident = new_id(&ast["ident"]);
    let args = ast["args"].as_array().unwrap().iter().map(new_id).collect();
    let body = convert_to_stmt(&ast["body"]);
    (ident, args, Rc::new(body))
}

fn convert_to_rest(ast: &Value) -> LinkedList<((i32, Identifier), Box<FruExpression>)> {
    ast.as_array()
        .unwrap()
        .iter()
        .map(|x| {
            let str = x.as_array().unwrap()[0].as_str().unwrap();
            let precedence = calculate_precedence(str);
            let ident = Identifier::new(str);
            let expr = convert_to_expr(&x.as_array().unwrap()[1]);

            ((precedence, ident), Box::new(expr))
        })
        .collect()
}
