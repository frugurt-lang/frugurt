use crate::common::{fru_err_res, FruError, FruValue, IdOfObject, Identifier};
use std::{cell::RefCell, collections::hash_map::Entry, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Clone)]
pub struct Thing {
    internal: Rc<ThingInternal>,
}

struct ThingInternal {
    props: RefCell<HashMap<Identifier, FruValue>>,
    prototype: Option<Thing>,
    uid: IdOfObject,
}

impl Thing {
    pub fn new() -> Thing {
        Thing {
            internal: Rc::new(ThingInternal {
                props: RefCell::default(),
                prototype: None,
                uid: IdOfObject::new(),
            }),
        }
    }

    pub fn derive_new(&self) -> Thing {
        Thing {
            internal: Rc::new(ThingInternal {
                props: RefCell::default(),
                prototype: Some(self.clone()),
                uid: IdOfObject::new(),
            }),
        }
    }

    pub fn get_uid(&self) -> IdOfObject {
        self.internal.uid
    }

    pub fn get_prototype(&self) -> Option<Thing> {
        self.internal.prototype.clone()
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(prop) = self.internal.props.borrow().get(&ident) {
            Ok(prop.clone())
        } else if let Some(proto) = &self.internal.prototype {
            proto.get_prop(ident)
        } else {
            fru_err_res!("prop `{}` does not exist", ident)
        }
    }

    pub fn let_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self.internal.props.borrow_mut().entry(ident) {
            Entry::Occupied(_) => {
                fru_err_res!("prop `{:?}` already exists", ident)
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        match self.internal.props.borrow_mut().entry(ident) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(_) => {
                if let Some(proto) = &self.internal.prototype {
                    proto.set_prop(ident, value)
                } else {
                    fru_err_res!("prop `{:?}` does not exist", ident)
                }
            }
        }
    }

    pub fn has_prop(&self, ident: Identifier) -> bool {
        self.internal.props.borrow().contains_key(&ident)
    }
}

impl Debug for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "thing<{:?}>", self.get_uid())
    }
}
