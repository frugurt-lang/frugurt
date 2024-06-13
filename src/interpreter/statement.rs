use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::interpreter::{
    control::Control,
    expression::FruExpression,
    identifier::{Identifier, OperatorIdentifier},
    scope::Scope,
    value::fru_type::{FruField, FruType, Property, TypeType},
    value::fru_value::FruValue,
    value::function::{FormalParameters, FruFunction},
    value::operator::AnyOperator,
};
use crate::stdlib::scope::fru_scope::extract_scope_from_value;

// TODO: make normal struct
pub type RawMethods = Vec<(bool, Identifier, FormalParameters, Rc<FruStatement>)>;

#[derive(Debug, Clone)]
pub enum FruStatement {
    SourceCode {
        body: Vec<FruStatement>,
    },
    Block {
        body: Vec<FruStatement>,
    },
    ScopeModifier {
        what: Box<FruExpression>,
        body: Vec<FruStatement>,
    },
    Expression {
        value: Box<FruExpression>,
    },
    Let {
        ident: Identifier,
        value: Box<FruExpression>,
    },
    Set {
        ident: Identifier,
        value: Box<FruExpression>,
    },
    SetProp {
        what: Box<FruExpression>,
        ident: Identifier,
        value: Box<FruExpression>,
    },
    If {
        condition: Box<FruExpression>,
        then_body: Box<FruStatement>,
        else_body: Option<Box<FruStatement>>,
    },
    While {
        condition: Box<FruExpression>,
        body: Box<FruStatement>,
    },
    Return {
        value: Option<Box<FruExpression>>,
    },
    Break,
    Continue,
    Operator {
        ident: Identifier,
        commutative: bool,
        left_ident: Identifier,
        left_type_ident: Identifier,
        right_ident: Identifier,
        right_type_ident: Identifier,
        body: Rc<FruStatement>,
    },
    Type {
        type_type: TypeType,
        ident: Identifier,
        fields: Vec<FruField>,
        static_fields: Vec<(FruField, Option<Box<FruExpression>>)>,
        properties: HashMap<Identifier, Property>,
        static_properties: HashMap<Identifier, Property>,
        methods: RawMethods,
    },
}

impl FruStatement {
    pub fn execute(&self, scope: Rc<Scope>) -> Result<(), Control> {
        match self {
            FruStatement::SourceCode { body } => {
                for statement in body {
                    statement.execute(scope.clone())?;
                }
            }

            FruStatement::Block { body } => {
                let new_scope = Scope::new_with_parent(scope.clone());

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }
            }

            FruStatement::ScopeModifier { what, body } => {
                let what = what.evaluate(scope)?;
                let new_scope = match extract_scope_from_value(&what) {
                    Some(x) => x,
                    None => {
                        return Control::new_err(format!(
                            "Expected `Scope` in scope modifier statement, got `{}`",
                            what.get_type_identifier()
                        ))
                    }
                };

                for statement in body {
                    statement.execute(new_scope.clone())?;
                }
            }

            FruStatement::Expression { value } => {
                value.evaluate(scope.clone())?;
            }

            FruStatement::Let { ident, value } => {
                let v = value.evaluate(scope.clone())?;

                scope.let_variable(*ident, v.fru_clone())?;
            }

            FruStatement::Set { ident, value } => {
                let v = value.evaluate(scope.clone())?;

                scope.set_variable(*ident, v.fru_clone())?;
            }

            FruStatement::SetProp { what, ident, value } => {
                let t = what.evaluate(scope.clone())?;
                let v = value.evaluate(scope.clone())?;
                t.set_prop(*ident, v.fru_clone())?;
            }

            FruStatement::If {
                condition,
                then_body,
                else_body,
            } => {
                let result = condition.evaluate(scope.clone())?;

                match result {
                    FruValue::Bool(true) => then_body.execute(scope.clone())?,

                    FruValue::Bool(false) => {
                        if let Some(else_body) = else_body {
                            else_body.execute(scope.clone())?
                        }
                    }

                    _ => {
                        return Control::new_err(format!(
                            "Expected `Bool` in if condition, got `{}`",
                            result.get_type_identifier()
                        ));
                    }
                }
            }

            FruStatement::While { condition, body } => {
                while {
                    match condition.evaluate(scope.clone())? {
                        FruValue::Bool(b) => b,
                        other => {
                            return Control::new_err(format!(
                                "Expected `Bool` in while condition, got `{}`",
                                other.get_type_identifier()
                            ));
                        }
                    }
                } {
                    if let Err(signal) = body.execute(scope.clone()) {
                        match signal {
                            Control::Continue => continue,
                            Control::Break => break,
                            Control::Return(v) => return Err(Control::Return(v)),
                            Control::Error(err) => return Err(Control::Error(err)),
                        }
                    }
                }
            }

            FruStatement::Return { value } => {
                return Err(Control::Return(match value {
                    Some(x) => x.evaluate(scope)?,
                    None => FruValue::Nah,
                }));
            }

            FruStatement::Break => return Err(Control::Break),
            FruStatement::Continue => return Err(Control::Continue),

            FruStatement::Operator {
                ident,
                commutative,
                left_ident,
                left_type_ident,
                right_ident,
                right_type_ident,
                body,
            } => {
                if *commutative {
                    scope.set_operator(
                        OperatorIdentifier::new(*ident, *right_type_ident, *left_type_ident),
                        AnyOperator::Operator {
                            left_ident: *right_ident,
                            right_ident: *left_ident,
                            body: body.clone(),
                            scope: scope.clone(),
                        }
                        .clone(),
                    );
                }

                scope.set_operator(
                    OperatorIdentifier::new(*ident, *left_type_ident, *right_type_ident),
                    AnyOperator::Operator {
                        left_ident: *left_ident,
                        right_ident: *right_ident,
                        body: body.clone(),
                        scope: scope.clone(),
                    },
                );
            }

            FruStatement::Type {
                type_type,
                ident,
                fields,
                static_fields,
                properties,
                static_properties,
                methods,
            } => {
                let mut methods_ = HashMap::new();
                let mut static_methods_ = HashMap::new();

                for (is_static, ident, arg_list, body) in methods {
                    let mt = FruFunction {
                        argument_idents: arg_list.clone(),
                        body: body.clone(),
                        scope: scope.clone(),
                    };
                    if *is_static {
                        static_methods_.insert(*ident, mt);
                    } else {
                        methods_.insert(*ident, mt);
                    }
                }

                let mut static_fields_evaluated = HashMap::new();
                for (field, value) in static_fields {
                    let value = if let Some(v) = value {
                        v.evaluate(scope.clone())?
                    } else {
                        FruValue::Nah
                    };

                    static_fields_evaluated.insert(field.ident, value);
                }

                scope.let_variable(
                    *ident,
                    FruType::new_value(
                        *ident,
                        *type_type,
                        fields.clone(),
                        RefCell::new(static_fields_evaluated),
                        properties.clone(),
                        static_properties.clone(),
                        methods_,
                        static_methods_,
                        scope.clone(),
                    ),
                )?;
            }
        }

        Ok(())
    }
}
