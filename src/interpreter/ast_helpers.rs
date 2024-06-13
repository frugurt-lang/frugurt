use std::rc::Rc;

use crate::interpreter::{
    expression::FruExpression, identifier::Identifier, statement::FruStatement,
    value::function::FormalParameters,
};

#[derive(Debug, Clone)]
pub struct RawStaticField {
    pub ident: Identifier,
    pub value: Option<Box<FruExpression>>,
}

#[derive(Debug, Clone)]
pub struct RawMethod {
    pub is_static: bool,
    pub ident: Identifier,
    pub parameters: FormalParameters,
    pub body: Rc<FruStatement>,
}
