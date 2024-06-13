use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::{returned, returned_nothing},
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    value::fru_type::FruType,
    value::fru_type::TypeType,
    value::fru_value::FruValue,
    value::function::FruFunction,
};

#[derive(Clone)]
pub struct FruObject {
    internal: Rc<FruObjectInternal>,
}

pub struct FruObjectInternal {
    type_: FruType,
    fields: RefCell<Vec<FruValue>>,
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

    fn get_kth_field(&self, i: usize) -> FruValue {
        self.internal.fields.borrow()[i].clone()
    }

    fn set_kth_field(&self, i: usize, value: FruValue) {
        self.internal.fields.borrow_mut()[i] = value
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(k) = self.get_type().get_field_k(ident) {
            return Ok(self.get_kth_field(k));
        }

        if let Some(property) = self.get_type().get_property(ident) {
            let new_scope = Scope::new_with_object(self.clone());

            return match property.getter {
                Some(getter) => returned(getter.evaluate(new_scope)),

                None => FruError::new_res(format!("property `{}` has no getter", ident)),
            };
        }

        if let Some(FruFunction {
            argument_idents,
            body,
            ..
        }) = self.get_type().get_method(ident)
        {
            return Ok(FruFunction {
                argument_idents,
                body,
                scope: Scope::new_with_object(self.clone()),
            }
            .into());
        }

        if let Ok(static_thing) = self.get_type().get_prop(ident) {
            return Ok(static_thing);
        }

        FruError::new_res(format!("prop `{}` not found", ident))
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        if let Some(field_k) = self.get_type().get_field_k(ident) {
            if self.get_type().get_type_type() == TypeType::Data {
                return FruError::new_res(format!(
                    "cannot set field `{}` in 'data' type `{}`",
                    ident,
                    value.get_type_identifier()
                ));
            }

            self.set_kth_field(field_k, value);
            return Ok(());
        }

        if let Some(property) = self.get_type().get_property(ident) {
            return if let Some((ident, setter)) = property.setter {
                let new_scope = Scope::new_with_object(self.clone());

                new_scope.let_variable(ident, value)?;

                returned_nothing(setter.execute(new_scope))
            } else {
                FruError::new_res(format!("property `{}` has no setter", ident))
            };
        }

        if let Ok(()) = self.get_type().set_prop(ident, value) {
            return Ok(());
        }

        FruError::new_res(format!(
            "prop `{}` does not exist in struct `{}`",
            ident,
            self.get_type().get_ident()
        ))
    }

    pub fn fru_clone(&self) -> FruValue {
        let tt = self.get_type().get_type_type();

        match tt {
            TypeType::Struct => FruObject::new_object(
                self.get_type(),
                self.internal.fields.borrow().iter().map(FruValue::fru_clone).collect(),
            ),

            TypeType::Class | TypeType::Data => FruValue::Object(self.clone()),
        }
    }
}

impl PartialEq for FruObject {
    fn eq(&self, other: &Self) -> bool {
        if self.get_type() != other.get_type() {
            return false;
        }

        self.internal.fields == other.internal.fields
    }
}

impl Debug for FruObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{{", self.get_type())?;

        let fields = self.get_type().get_fields().len();

        for (k, (field, value)) in self
            .get_type()
            .get_fields()
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
