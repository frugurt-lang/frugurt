use std::{cell::RefCell, fmt::Debug, rc::Rc};

use uid::Id;

use crate::{
    fru_err_res,
    interpreter::{
        control::{returned, returned_nothing},
        error::FruError,
        identifier::Identifier,
        scope::Scope,
        value::{
            fru_function::FruFunction, fru_type::FruType, fru_type::TypeFlavor,
            fru_value::FruValue, native_object::OfObject,
        },
    },
};

#[derive(Clone)]
pub struct FruObject {
    internal: Rc<FruObjectInternal>,
}

pub struct FruObjectInternal {
    type_: FruType,
    fields: RefCell<Vec<FruValue>>,
    uid: Id<OfObject>,
}

impl FruObject {
    pub fn new(type_: FruType, fields: Vec<FruValue>) -> FruObject {
        FruObject {
            internal: Rc::new(FruObjectInternal {
                type_,
                fields: RefCell::new(fields),
                uid: Id::new(),
            }),
        }
    }

    pub fn new_object(type_: FruType, fields: Vec<FruValue>) -> FruValue {
        FruValue::Object(FruObject::new(type_, fields))
    }

    pub fn get_type(&self) -> FruValue {
        FruValue::Type(self.get_fru_type())
    }

    pub fn get_uid(&self) -> Id<OfObject> {
        self.internal.uid
    }

    pub fn get_fru_type(&self) -> FruType {
        self.internal.type_.clone()
    }

    fn get_kth_field(&self, i: usize) -> FruValue {
        self.internal.fields.borrow()[i].clone()
    }

    fn set_kth_field(&self, i: usize, value: FruValue) {
        self.internal.fields.borrow_mut()[i] = value
    }

    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(k) = self.get_fru_type().get_field_k(ident) {
            return Ok(self.get_kth_field(k));
        }

        if let Some(property) = self.get_fru_type().get_property(ident) {
            let new_scope = Scope::new_with_object(self.clone());

            return match property.getter {
                Some(getter) => returned(getter.evaluate(new_scope)),

                None => fru_err_res!("property `{}` has no getter", ident),
            };
        }

        if let Some(f) = self.get_fru_type().get_method(ident) {
            return Ok(FruValue::Function(Rc::new(FruFunction {
                parameters: f.parameters,
                body: f.body,
                scope: Scope::new_with_object(self.clone()),
            })));
        }

        if let Ok(static_thing) = self.get_fru_type().get_prop(ident) {
            return Ok(static_thing);
        }

        fru_err_res!("prop `{}` not found", ident)
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        if let Some(field_k) = self.get_fru_type().get_field_k(ident) {
            if self.get_fru_type().get_type_flavor() == TypeFlavor::Data {
                return fru_err_res!(
                    "cannot set field `{}` in 'data' type `{:?}`",
                    ident,
                    value.get_type()
                );
            }

            self.set_kth_field(field_k, value);
            return Ok(());
        }

        if let Some(property) = self.get_fru_type().get_property(ident) {
            return if let Some((ident, setter)) = property.setter {
                let new_scope = Scope::new_with_object(self.clone());

                new_scope.let_variable(ident, value)?;

                returned_nothing(setter.execute(new_scope))
            } else {
                fru_err_res!("property `{}` has no setter", ident)
            };
        }

        if let Ok(()) = self.get_fru_type().set_prop(ident, value) {
            return Ok(());
        }

        fru_err_res!(
            "prop `{}` does not exist in struct `{}`",
            ident,
            self.get_fru_type().get_ident()
        )
    }

    pub fn fru_clone(&self) -> FruValue {
        let tt = self.get_fru_type().get_type_flavor();

        match tt {
            TypeFlavor::Struct => FruObject::new_object(
                self.get_fru_type(),
                self.internal.fields.borrow().iter().map(FruValue::fru_clone).collect(),
            ),

            TypeFlavor::Class | TypeFlavor::Data => FruValue::Object(self.clone()),
        }
    }
}

impl PartialEq for FruObject {
    fn eq(&self, other: &Self) -> bool {
        if self.get_fru_type() != other.get_fru_type() {
            return false;
        }

        self.internal.fields == other.internal.fields
    }
}

impl Debug for FruObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}{{", self.get_fru_type())?;

        let fields = self.get_fru_type().get_fields().len();

        for (k, (field, value)) in self
            .get_fru_type()
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
