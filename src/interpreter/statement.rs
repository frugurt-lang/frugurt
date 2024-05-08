use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::interpreter::{
    control::Control,
    error::FruError,
    expression::FruExpression,
    identifier::{Identifier, OperatorIdentifier},
    scope::Scope,
    value::fru_type::{FruField, FruType, FruTypeInternal, TypeType},
    value::fru_value::FruValue,
    value::function::{FormalParameters, FruFunction},
    value::operator::AnyOperator,
};

#[derive(Debug, Clone)]
pub enum FruStatement {
    Block {
        body: Vec<FruStatement>
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
    SetField {
        what: Box<FruExpression>,
        field: Identifier,
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
        methods: Vec<(bool, Identifier, FormalParameters, Rc<FruStatement>)>,
    },
}

impl FruStatement {
    pub fn execute(&self, scope: Rc<Scope>) -> Result<(), Control> {
        match self {
            FruStatement::Block { body } => {
                let new_scope = Scope::new_with_parent(scope.clone());

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

            FruStatement::SetField {
                what,
                field,
                value,
            } => {
                let t = what.evaluate(scope.clone())?;
                let v = value.evaluate(scope.clone())?;
                t.set_field(*field, v.fru_clone())?;
            }

            FruStatement::If {
                condition,
                then_body,
                else_body,
            } => {
                let result = condition.evaluate(scope.clone())?;

                if let FruValue::Bool(b) = result {
                    if b {
                        then_body.execute(scope.clone())?;
                    } else if let Some(ref else_) = else_body {
                        else_.execute(scope.clone())?;
                    }
                } else {
                    return FruError::new_control(format!(
                        "Expected bool in if condition, got {}",
                        result.get_type_identifier()
                    ));
                }
            }

            FruStatement::While {
                condition,
                body,
            } => {
                while {
                    match condition.evaluate(scope.clone())? {
                        FruValue::Bool(b) => b,
                        other => {
                            return FruError::new_control(format!(
                                "unexpected value with type {:?} in while condition: {:?}",
                                other.get_type_identifier(),
                                other
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
                return Err(Control::Return(
                    match value {
                        Some(x) => x.evaluate(scope)?,
                        None => FruValue::Nah
                    }
                ));
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

                let internal = FruTypeInternal {
                    ident: *ident,
                    type_type: *type_type,
                    fields: fields.clone(),
                    static_fields: RefCell::new(static_fields_evaluated),
                    methods: methods_,
                    static_methods: static_methods_,
                    scope: scope.clone(),
                };

                scope.let_variable(*ident, FruType::new_value(internal))?;
            }
        }

        Ok(())
    }
}
