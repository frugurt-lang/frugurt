use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    value::fru_type::FruType,
    value::fru_value::FruValue,
    value::function::{AnyFunction, FruFunction},
    value::fru_type::TypeType,
};

#[derive(Clone)]
pub struct FruObject {
    internal: Rc<FruObjectInternal>,
}

pub struct FruObjectInternal {
    pub type_: FruType,
    pub fields: RefCell<Vec<FruValue>>,
}

impl FruObject {
    pub fn new(type_: FruType, fields: Vec<FruValue>) -> FruObject {
        FruObject {
            internal: Rc::new(FruObjectInternal {
                type_,
                fields: RefCell::new(fields),
            }),
        }
    }

    pub fn new_object(type_: FruType, fields: Vec<FruValue>) -> FruValue {
        FruValue::Object(FruObject::new(type_, fields))
    }

    pub fn get_type(&self) -> FruType {
        self.internal.type_.clone()
    }

    pub fn get_kth_field(&self, i: usize) -> FruValue {
        self.internal.fields.borrow()[i].clone()
    }

    pub fn set_kth_field(&self, i: usize, value: FruValue) {
        self.internal.fields.borrow_mut()[i] = value
    }

    pub fn get_field(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(k) = self.get_type().get_field_k(ident) {
            Ok(self.get_kth_field(k))
        } else if let Some(method) = self.get_type().get_method(ident) {
            Ok(FruValue::Function(AnyFunction::Function(Rc::new(
                FruFunction {
                    argument_idents: method.argument_idents,
                    body: method.body,
                    scope: Scope::new_with_object_then_parent(self.clone(), method.scope),
                },
            ))))
        } else if let Ok(static_thing) = self.get_type().get_field(ident) {
            Ok(static_thing)
        } else {
            FruError::new_val(format!("field or method {} not found", ident))
        }
    }

    pub fn set_field(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        let pos = self.get_type().get_field_k(ident);

        let pos = match pos {
            Some(p) => p,
            None => {
                return FruError::new_unit(format!(
                    "field {} does not exist in struct {}",
                    ident,
                    self.get_type().get_ident()
                ));
            }
        };

        self.set_kth_field(pos, value);

        for watch in self.get_type().get_watches_by_field(ident) {
            let scope =
                Scope::new_with_object_then_parent(self.clone(), self.get_type().get_scope());
            watch.run(scope)?;
        }

        Ok(())
    }

    pub fn fru_clone(&self) -> FruValue {
        let tt = self.get_type().get_type_type();

        if tt == TypeType::Struct {
            FruObject::new_object(
                self.get_type().clone(),
                self.internal
                    .fields
                    .borrow()
                    .iter()
                    .map(|x| x.fru_clone())
                    .collect(),
            )
        } else {
            unimplemented!();
        }
    }
}

impl PartialEq for FruObject {
    fn eq(&self, other: &Self) -> bool {
        if self.get_type().get_type_type() == TypeType::Struct {
            self.internal.fields == other.internal.fields
        } else {
            unimplemented!()
        }
    }
}

impl Debug for FruObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{{", self.get_type())?;

        let fields = self.get_type().get_internal().fields.len();

        for (k, (field, value)) in self
            .get_type()
            .get_internal()
            .fields
            .iter()
            .zip(self.internal.fields.borrow().iter())
            .enumerate()
        {
            write!(f, "{:?}={:?}", field, value)?;

            if k + 1 < fields {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}")
    }
}
