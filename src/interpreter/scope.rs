use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use uid::Id;

use crate::{
    fru_err_res,
    interpreter::{
        error::FruError,
        identifier::Identifier,
        value::{
            fru_object::FruObject, fru_type::FruType, fru_value::FruValue, native_object::OfObject,
        },
    },
    stdlib::prelude::variables::builtin_variables,
};

pub struct Scope {
    variables: RefCell<HashMap<Identifier, FruValue>>,
    parent: ScopeAncestor,
    pub uid: Id<OfObject>,
}

enum ScopeAncestor {
    None,
    Parent(Rc<Scope>),
    Object {
        object: FruObject,
        parent: Rc<Scope>,
    },
    Type {
        type_: FruType,
        parent: Rc<Scope>,
    },
}

impl Scope {
    pub fn new_global() -> Rc<Scope> {
        Rc::new(Scope {
            variables: RefCell::new(builtin_variables()),
            parent: ScopeAncestor::None,
            uid: Id::new(),
        })
    }

    pub fn new_with_parent(parent: Rc<Scope>) -> Rc<Scope> {
        Rc::new(Scope {
            variables: RefCell::new(HashMap::new()),
            parent: ScopeAncestor::Parent(parent),
            uid: Id::new(),
        })
    }

    pub fn new_with_object(object: FruObject) -> Rc<Scope> {
        let parent = object.get_fru_type().get_scope();

        Rc::new(Scope {
            variables: RefCell::new(HashMap::new()),
            parent: ScopeAncestor::Object { object, parent },
            uid: Id::new(),
        })
    }

    pub fn new_with_type(type_: FruType) -> Rc<Scope> {
        let parent = type_.get_scope();

        Rc::new(Scope {
            variables: RefCell::new(HashMap::new()),
            parent: ScopeAncestor::Type { type_, parent },
            uid: Id::new(),
        })
    }

    pub fn get_variable(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(var) = self.variables.borrow().get(&ident) {
            Ok(var.clone())
        } else {
            self.parent.get_variable(ident)
        }
    }

    pub fn let_variable(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self.variables.borrow_mut().entry(ident) {
            Entry::Occupied(_) => {
                fru_err_res!("variable `{:?}` already exists", ident)
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn set_variable(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self.variables.borrow_mut().entry(ident) {
            Entry::Occupied(entry) => {
                *entry.into_mut() = value;
                Ok(())
            }
            Entry::Vacant(_) => self.parent.set_variable(ident, value),
        }
    }

    pub fn has_variable(&self, ident: Identifier) -> bool {
        self.variables.borrow().contains_key(&ident)
    }

    pub fn let_set_variable(&self, ident: Identifier, value: FruValue) {
        self.variables.borrow_mut().insert(ident, value);
    }
}

impl ScopeAncestor {
    fn get_variable(&self, ident: Identifier) -> Result<FruValue, FruError> {
        match self {
            ScopeAncestor::None => {
                fru_err_res!("variable `{:?}` does not exist", ident)
            }
            ScopeAncestor::Parent(parent) => parent.get_variable(ident),
            ScopeAncestor::Object { object, parent } => {
                object.get_prop(ident).or_else(|_| parent.get_variable(ident))
            }
            ScopeAncestor::Type { type_, parent } => {
                type_.get_prop(ident).or_else(|_| parent.get_variable(ident))
            }
        }
    }

    fn set_variable(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self {
            ScopeAncestor::None => {
                fru_err_res!("variable `{:?}` does not exist", ident)
            }

            ScopeAncestor::Parent(parent) => parent.set_variable(ident, value),

            ScopeAncestor::Object { object, parent } => object
                .set_prop(ident, value.clone())
                .or_else(|_| parent.set_variable(ident, value)),

            ScopeAncestor::Type { type_, parent } => type_
                .set_prop(ident, value.clone())
                .or_else(|_| parent.set_variable(ident, value)),
        }
    }
}
