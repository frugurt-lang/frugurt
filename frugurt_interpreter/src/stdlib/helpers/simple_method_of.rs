use std::{any::Any, fmt::Debug, rc::Rc};

use crate::common::{
    EvaluatedArgumentList, FruError, FruValue, INativeObject, IdOfObject, Identifier,
};
use crate::value::fru_value::type_id;

type SimpleMethodOfFn<T> = fn(&Rc<T>, EvaluatedArgumentList) -> Result<FruValue, FruError>;

pub struct SimpleMethodOf<T: INativeObject> {
    ident: Identifier,
    owner: Rc<T>,
    fun: SimpleMethodOfFn<T>,
    uid: IdOfObject,
}

impl<T: INativeObject> SimpleMethodOf<T> {
    pub fn new(ident: Identifier, owner: Rc<T>, fun: SimpleMethodOfFn<T>) -> Self {
        Self {
            ident,
            owner,
            fun,
            uid: IdOfObject::new(),
        }
    }
}

impl<T: INativeObject + 'static> INativeObject for SimpleMethodOf<T> {
    fn as_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

    fn get_uid(&self) -> IdOfObject {
        self.uid
    }

    fn get_type_uid(&self) -> IdOfObject {
        *type_id::FUNCTION_TYPE_ID
    }

    fn call(self: Rc<Self>, _args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        (self.fun)(&self.owner, _args)
    }
}

impl<T: INativeObject> Debug for SimpleMethodOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}.{}", self.owner, self.ident)
    }
}
