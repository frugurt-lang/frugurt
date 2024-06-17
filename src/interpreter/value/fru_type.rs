use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::interpreter::{
    control::{returned, returned_nothing},
    error::FruError,
    expression::FruExpression,
    helpers::WrappingExtension,
    identifier::Identifier,
    scope::Scope,
    statement::FruStatement,
    value::fru_object::FruObject,
    value::fru_value::FruValue,
    value::function::{EvaluatedArgumentList, FruFunction},
};

#[derive(Clone)]
pub struct FruType {
    internal: Rc<FruTypeInternal>,
}

#[derive(Clone)]
pub struct FruTypeInternal {
    ident: Identifier,
    type_type: TypeType,
    fields: Vec<FruField>,
    static_fields: RefCell<HashMap<Identifier, FruValue>>,
    // TODO: change for FruField?
    properties: HashMap<Identifier, Property>,
    static_properties: HashMap<Identifier, Property>,
    methods: HashMap<Identifier, FruFunction>,
    static_methods: HashMap<Identifier, FruFunction>,
    scope: Rc<Scope>,
}

#[derive(Clone)]
pub struct FruField {
    pub is_public: bool,
    pub ident: Identifier,
    pub type_ident: Option<Identifier>, // useless for now
}

#[derive(Debug, Clone)]
pub struct Property {
    pub ident: Identifier,
    pub getter: Option<Rc<FruExpression>>,
    pub setter: Option<(Identifier, Rc<FruStatement>)>, // ident for value variable
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeType {
    Struct,
    Class,
    Data,
}

impl FruType {
    pub fn new_value(
        ident: Identifier,
        type_type: TypeType,
        fields: Vec<FruField>,
        static_fields: RefCell<HashMap<Identifier, FruValue>>,
        properties: HashMap<Identifier, Property>,
        static_properties: HashMap<Identifier, Property>,
        methods: HashMap<Identifier, FruFunction>,
        static_methods: HashMap<Identifier, FruFunction>,
        scope: Rc<Scope>,
    ) -> FruValue {
        FruValue::Type(Self {
            internal: FruTypeInternal {
                ident,
                type_type,
                fields,
                static_fields,
                properties,
                methods,
                static_methods,
                static_properties,
                scope,
            }
            .wrap_rc(),
        })
    }

    pub fn get_ident(&self) -> Identifier {
        self.internal.ident
    }

    pub fn get_type_type(&self) -> TypeType {
        self.internal.type_type
    }

    pub fn get_scope(&self) -> Rc<Scope> {
        self.internal.scope.clone()
    }

    pub fn get_fields(&self) -> &[FruField] {
        self.internal.fields.as_slice()
    }

    pub fn get_field_k(&self, ident: Identifier) -> Option<usize> {
        for (i, field_ident) in self.internal.fields.iter().enumerate() {
            if field_ident.ident == ident {
                return Some(i);
            }
        }
        None
    }

    pub fn get_property(&self, ident: Identifier) -> Option<Property> {
        self.internal.properties.get(&ident).cloned()
    }

    pub fn get_method(&self, ident: Identifier) -> Option<FruFunction> {
        self.internal.methods.get(&ident).cloned()
    }

    /// In this case means static field of method
    pub fn get_prop(&self, ident: Identifier) -> Result<FruValue, FruError> {
        if let Some(field) = self.internal.static_fields.borrow().get(&ident) {
            return Ok(field.clone());
        }

        if let Some(property) = self.internal.static_properties.get(&ident) {
            let new_scope = Scope::new_with_type(self.clone());

            return match &property.getter {
                Some(getter) => returned(getter.evaluate(new_scope)),

                None => FruError::new_res(format!("static property `{}` has no getter", ident)),
            };
        }

        if let Some(static_method) = self.internal.static_methods.get(&ident) {
            return Ok(FruFunction {
                parameters: static_method.parameters.clone(),
                body: static_method.body.clone(),
                scope: Scope::new_with_type(self.clone()),
            }
            .into());
        }

        FruError::new_res(format!("static prop `{}` not found", ident))
    }

    pub fn set_prop(&self, ident: Identifier, value: FruValue) -> Result<(), FruError> {
        if let Some(field) = self.internal.static_fields.borrow_mut().get_mut(&ident) {
            *field = value;
            return Ok(());
        }

        if let Some(property) = self.internal.static_properties.get(&ident) {
            return match &property.setter {
                Some((ident, setter)) => {
                    let new_scope = Scope::new_with_type(self.clone());

                    new_scope.let_variable(*ident, value)?;

                    returned_nothing(setter.execute(new_scope))
                }

                None => FruError::new_res(format!("static property `{}` has no setter", ident)),
            };
        }

        FruError::new_res(format!("static prop `{}` not found", ident))
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
                return FruError::new_res(format!("field `{}` is set more than once", ident));
            }
            obj_fields.insert(ident, value);
        }

        let mut args = Vec::new();

        for FruField { ident, .. } in fields {
            match obj_fields.remove(ident) {
                Some(value) => args.push(value),
                None => return FruError::new_res(format!("missing field `{}`", ident)),
            }
        }

        if let Some(ident) = obj_fields.keys().next() {
            return FruError::new_res(format!("field `{}` does not exist", *ident));
        }

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
