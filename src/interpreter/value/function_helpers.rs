use std::{collections::HashSet, rc::Rc};

use crate::interpreter::{
    control::returned, error::FruError, expression::FruExpression, identifier::Identifier,
    scope::Scope, value::fru_value::FruValue,
};

#[derive(Clone, Copy, Debug)]
pub enum ArgumentError {
    TooMany,
    SameSetTwice {
        ident: Identifier,
    },
    NotSetPositional {
        ident: Identifier,
    },
    DoesNotExist {
        ident: Identifier,
    },
}

#[derive(Clone, Debug)]
pub struct FormalParameters {
    pub args: Vec<(Identifier, Option<FruExpression>)>,
}

#[derive(Clone, Debug)]
pub struct ArgumentList {
    pub args: Vec<(Option<Identifier>, FruExpression)>,
}

#[derive(Clone, Debug)]
pub struct EvaluatedArgumentList {
    pub args: Vec<(Option<Identifier>, FruValue)>,
}

impl FormalParameters {
    // scope is the scope of function being called
    pub fn apply(
        &self,
        evaluated: EvaluatedArgumentList,
        scope: Rc<Scope>,
    ) -> Result<(), FruError> {
        let mut next_positional = 0;

        let acceptable: HashSet<_> = self.args.iter().map(|(x, _)| *x).collect();

        for (ident, value) in evaluated.args {
            let ident = match ident {
                Some(ident) => {
                    if !acceptable.contains(&ident) {
                        return Err(ArgumentError::DoesNotExist { ident }.into());
                    }
                    ident
                }
                None => {
                    if next_positional >= self.args.len() {
                        return Err(ArgumentError::TooMany.into());
                    }
                    let r = self.args[next_positional].0;
                    next_positional += 1;
                    r
                }
            };

            scope
                .let_variable(ident, value)
                .map_err(|_| ArgumentError::SameSetTwice { ident })?;
        }

        for (ident, value) in self.args.iter().skip(next_positional) {
            if scope.has_variable(*ident) {
                continue;
            }

            if let Some(default) = value {
                let default = returned(default.evaluate(scope.clone()))?;

                scope.let_variable(*ident, default)?;
            } else {
                return Err(ArgumentError::NotSetPositional { ident: *ident }.into());
            }
        }

        Ok(())
    }
}
