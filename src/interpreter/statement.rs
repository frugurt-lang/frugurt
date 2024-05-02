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
    value::fru_type::TypeType,
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
        watches: Vec<(Vec<Identifier>, Rc<FruStatement>)>,
        methods: Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>,
        static_methods: Vec<(Identifier, Vec<Identifier>, Rc<FruStatement>)>,
    },
}

impl FruStatement {
    pub fn execute(&self, scope: Rc<Scope>) -> Control {
        match self {
            FruStatement::Block { body } => {
                let new_scope = Scope::new_with_parent(scope.clone());

                for statement in body {
                    match statement.execute(new_scope.clone()) {
                        Control::Nah => {}

                        code => return code,
                    }
                }

                Control::Nah
            }

            FruStatement::Expression { value } => {
                value.evaluate(scope.clone())?;
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
                what,
                field,
                value,
            } => {
                let t = what.evaluate(scope.clone())?;
                let v = value.evaluate(scope.clone())?;
                t.set_field(*field, v.fru_clone())?;
                Control::Nah
            }

            FruStatement::If {
                condition,
                then_body,
                else_body,
            } => {
                let result = condition.evaluate(scope.clone())?;

                if let FruValue::Bool(b) = result {
                    if b {
                        then_body.execute(scope.clone())
                    } else if let Some(ref else_) = else_body {
                        else_.execute(scope.clone())
                    } else {
                        Control::Nah
                    }
                } else {
                    FruError::new_control(format!(
                        "{} is not a boolean",
                        result.get_type_identifier()
                    ))
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
                    let res = body.execute(scope.clone());

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
                Control::Return(
                    match value {
                        Some(x) => x.evaluate(scope)?,
                        None => FruValue::Nah
                    }
                )
            }

            FruStatement::Break => Control::Break,
            FruStatement::Continue => Control::Continue,

            FruStatement::Operator {
                ident,
                commutative,
                left_ident,
                left_type_ident,
                right_ident,
                right_type_ident,
                body,
            } => {
                if *commutative && *left_type_ident == *right_type_ident {
                    return FruError::new_control(format!(
                        "commutative operators must have different types, but {} was used twice",
                        *left_type_ident
                    ));
                }

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

                Control::Nah
            }
            FruStatement::Type {
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
