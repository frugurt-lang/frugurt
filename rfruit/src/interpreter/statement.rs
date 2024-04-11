use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::interpreter::{
    control::Control,
    easter_eggs,
    error::FruError,
    expression::FruExpression,
    identifier::{Identifier, OperatorIdentifier},
    scope::Scope,
    value::fru_type::{FruField, FruType, FruTypeInternal},
    value::fru_value::FruValue,
    value::fru_watch::FruWatch,
    value::function::FruFunction,
    value::operator::AnyOperator,
};

#[derive(Debug, Clone)]
pub enum FruStatement {
    Block(Vec<FruStatement>),
    Nothing,
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
        target: Box<FruExpression>,
        field: Identifier,
        value: Box<FruExpression>,
    },
    If {
        cond: Box<FruExpression>,
        then_body: Box<FruStatement>,
        else_body: Box<FruStatement>,
    },
    While {
        cond: Box<FruExpression>,
        body: Box<FruStatement>,
    },
    Return {
        value: Box<FruExpression>,
    },
    Break,
    Continue,
    OperatorDefinition {
        ident: Identifier,
        commutative: bool,
        left_ident: Identifier,
        left_type_ident: Identifier,
        right_ident: Identifier,
        right_type_ident: Identifier,
        body: Rc<FruStatement>,
    },
    TypeDeclaration {
        type_type: TypeType,
        ident: Identifier,
        fields: Vec<FruField>,
        static_fields: Vec<(FruField, Option<Box<FruExpression>>)>,
        watches: Vec<(Vec<Identifier>, Rc<FruStatement>)>,
        methods: Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>,
        static_methods: Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeType {
    Struct,
    // Class, TODO
    // Data
}

impl FruStatement {
    pub fn execute(&self, scope: Rc<Scope>) -> Control {
        match self {
            FruStatement::Block(statements) => {
                let new_scope = Scope::new_with_parent(scope.clone());

                for statement in statements {
                    match statement.execute(new_scope.clone()) {
                        Control::Nah => {}

                        code => return code,
                    }
                }

                Control::Nah
            }

            FruStatement::Nothing => Control::Nah,

            FruStatement::Expression { value: expr } => {
                expr.evaluate(scope.clone())?;
                Control::Nah
            }

            FruStatement::Let { ident, value } => {
                let v = value.evaluate(scope.clone())?;
                scope.let_variable(*ident, v.fru_clone())?;
                Control::Nah
            }

            FruStatement::Set { ident, value } => {
                let v = value.evaluate(scope.clone())?;
                scope.set_variable(*ident, v.fru_clone())?;
                Control::Nah
            }

            FruStatement::SetField {
                target,
                field,
                value,
            } => {
                let t = target.evaluate(scope.clone())?;
                let v = value.evaluate(scope.clone())?;
                t.set_field(*field, v.fru_clone())?;
                Control::Nah
            }

            FruStatement::If {
                cond: condition,
                then_body: then,
                else_body: else_,
            } => {
                let result = condition.evaluate(scope.clone())?;

                if let FruValue::Bool(b) = result {
                    if b {
                        then.execute(scope.clone())
                    } else {
                        else_.execute(scope.clone())
                    }
                } else {
                    FruError::new_control(format!(
                        "{} is not a boolean",
                        result.get_type_identifier()
                    ))
                }
            }

            FruStatement::While {
                cond: condition,
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
                    let res = body.execute(scope.clone())?;

                    match res {
                        Control::Nah => {}
                        Control::Continue => continue,
                        Control::Break => break,
                        Control::Return(v) => return Control::Return(v),
                        Control::Error(err) => return Control::Error(err),
                    }
                }

                Control::Nah
            }

            FruStatement::Return { value } => {
                let v = value.evaluate(scope)?;
                Control::Return(v)
            }

            FruStatement::Break => Control::Break,
            FruStatement::Continue => Control::Continue,

            FruStatement::OperatorDefinition {
                ident,
                commutative,
                left_ident: left_arg,
                left_type_ident: left_type,
                right_ident: right_arg,
                right_type_ident: right_type,
                body,
            } => {
                if *commutative && *left_type == *right_type {
                    return FruError::new_control(format!(
                        "commutative operators must have different types, but {} was used twice",
                        *left_type
                    ));
                }

                if *commutative {
                    scope.set_operator(
                        OperatorIdentifier::new(*ident, *right_type, *left_type),
                        AnyOperator::Operator {
                            left_ident: *right_arg,
                            right_ident: *left_arg,
                            body: body.clone(),
                            scope: scope.clone(),
                        }
                        .clone(),
                    );
                }

                scope.set_operator(
                    OperatorIdentifier::new(*ident, *left_type, *right_type),
                    AnyOperator::Operator {
                        left_ident: *left_arg,
                        right_ident: *right_arg,
                        body: body.clone(),
                        scope: scope.clone(),
                    },
                );

                Control::Nah
            }
            FruStatement::TypeDeclaration {
                type_type,
                ident,
                fields,
                static_fields,
                watches,
                methods,
                static_methods,
            } => {
                if *ident == Identifier::new("NahIdWin") {
                    easter_eggs::launch_satoru();
                }

                let methods = methods
                    .iter()
                    .map(|(ident, args, body)| {
                        (
                            *ident,
                            FruFunction {
                                argument_idents: args.clone(),
                                body: body.clone(),
                                scope: scope.clone(),
                            },
                        )
                    })
                    .collect();

                let static_methods = static_methods
                    .iter()
                    .map(|(ident, args, body)| {
                        (
                            *ident,
                            FruFunction {
                                argument_idents: args.clone(),
                                body: body.clone(),
                                scope: scope.clone(),
                            },
                        )
                    })
                    .collect();

                let mut static_fields_evaluated = HashMap::new();
                for (field, value) in static_fields {
                    let value = if let Some(v) = value {
                        v.evaluate(scope.clone())?
                    } else {
                        FruValue::Nah
                    };

                    static_fields_evaluated.insert(field.ident, value);
                }

                let raw_watches: Vec<(Vec<Identifier>, FruWatch)> = watches
                    .iter()
                    .map(|(idents, body)| (idents.clone(), (FruWatch { body: body.clone() })))
                    .collect();

                let watches = raw_watches.iter().map(|(_, x)| x.clone()).collect();

                let mut watches_by_field: HashMap<_, Vec<FruWatch>> = HashMap::new();

                for (idents, watch) in raw_watches {
                    for ident in idents {
                        watches_by_field
                            .entry(ident)
                            .or_default()
                            .push(watch.clone());
                    }
                }

                let internal = FruTypeInternal {
                    ident: *ident,
                    type_type: *type_type,
                    fields: fields.clone(),
                    static_fields: RefCell::new(static_fields_evaluated),
                    watches_by_field,
                    watches,
                    methods,
                    static_methods,
                    scope: scope.clone(),
                };

                scope.let_variable(*ident, FruType::new_value(internal))?;

                Control::Nah
            }
        }
    }
}
