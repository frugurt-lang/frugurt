use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::interpreter::{
    error::FruError,
    identifier::Identifier,
    scope::Scope,
    value::fru_object::FruObject,
    value::fru_value::FruValue,
    value::fru_watch::FruWatch,
    value::function::{AnyFunction, EvaluatedArgumentList, FruFunction},
};

#[derive(Clone)]
pub struct FruType {
    internal: Rc<FruTypeInternal>,
}

#[derive(Clone)]
pub struct FruTypeInternal {
    pub ident: Identifier,
    pub type_type: TypeType,
    pub fields: Vec<FruField>,
    pub static_fields: RefCell<HashMap<Identifier, FruValue>>,
    // TODO: change for FruField?
    pub watches_by_field: HashMap<Identifier, Vec<FruWatch>>,
    pub watches: Vec<FruWatch>,
    pub methods: HashMap<Identifier, FruFunction>,
    pub static_methods: HashMap<Identifier, FruFunction>,
    pub scope: Rc<Scope>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FruField {
    pub is_public: bool,
    pub ident: Identifier,
    pub type_ident: Option<Identifier>, // useless for now
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeType {
    Struct,
    Class,
    Data,
}

impl FruType {
    pub fn new_value(internal: FruTypeInternal) -> FruValue {
        FruValue::Type(Self {
            internal: Rc::new(internal),
        })
    }

    pub fn get_ident(&self) -> Identifier {
        self.internal.ident
    }

    pub fn get_type_type(&self) -> TypeType {
        self.internal.type_type
    }

    pub fn get_fields(&self) -> &[FruField] {
        self.internal.fields.as_slice()
    }

    pub fn get_watches_by_field(&self, ident: Identifier) -> &[FruWatch] {
        self.internal
            .watches_by_field
            .get(&ident)
            .map_or_else(Default::default, Vec::as_slice)
    }

    pub fn get_scope(&self) -> Rc<Scope> {
        self.internal.scope.clone()
    }

    pub fn get_field_k(&self, ident: Identifier) -> Option<usize> {
        for (i, field_ident) in self.internal.fields.iter().enumerate() {
            if field_ident.ident == ident {
                return Some(i);
            }
        }
        None
    }

    pub fn get_method(&self, ident: Identifier) -> Option<FruFunction> {
        self.internal.methods.get(&ident).cloned()
    }

    /// In this case means static field of method
    pub fn get_field(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(field) = self.internal.static_fields.borrow().get(&ident) {
            return Ok(field.clone());
        }

        if let Some(static_method) = self.internal.static_methods.get(&ident) {
            return Ok(FruValue::Function(AnyFunction::Function(Rc::new(
                FruFunction {
                    argument_idents: static_method.argument_idents.clone(),
                    body: static_method.body.clone(),
                    scope: Scope::new_with_type_then_parent(
                        self.clone(),
                        self.internal.scope.clone(),
                    ),
                },
            ))));
        }

        FruError::new_val(format!("static field or method {} not found", ident))
    }

    pub fn set_field(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        if let Some(field) = self.internal.static_fields.borrow_mut().get_mut(&ident) {
            *field = value;
            Ok(())
        } else {
            FruError::new_unit(format!("static field {} not found", ident))
        }
    }

    pub fn instantiate(&self, mut args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let mut obj_fields = HashMap::new();

        let fields = self.get_fields();

        for (n, (ident, value)) in args.args.drain(..).enumerate() {
            let ident = match ident {
                Some(ident) => ident,
                None => fields[n].ident,
            };
            if obj_fields.contains_key(&ident) {
                return FruError::new_val(format!("field {} is set more than once", ident));
            }
            obj_fields.insert(ident, value);
        }

        let mut args = Vec::new();

        for FruField { ident, .. } in fields {
            args.push(obj_fields.remove(&ident).unwrap());
        }

        if let Some(ident) = obj_fields.keys().next() {
            return FruError::new_val(format!("unknown field {}", *ident));
        }

        // TODO: fire watches
        Ok(FruObject::new_object(self.clone(), args))
    }
}

impl PartialEq for FruType {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.internal, &other.internal)
    }
}

impl Debug for FruField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_public {
            write!(f, "pub ")?;
        }
        write!(f, "{}", self.ident)?;
        if let Some(type_ident) = &self.type_ident {
            write!(f, ": {}", type_ident)
        } else {
            Ok(())
        }
    }
}

impl Debug for FruType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.internal.ident)
    }
}
