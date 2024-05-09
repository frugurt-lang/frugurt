use std::{
    collections::HashMap,
    fmt::Debug,
    fmt::Display,
    hash::DefaultHasher,
    hash::Hash,
    hash::Hasher,
    sync::Mutex,
};

use once_cell::sync::Lazy;

// this map is used for Identifier visualization
static BACKWARDS_MAP: Lazy<Mutex<HashMap<u64, String>>> = Lazy::new(Default::default);

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Identifier {
    // holds hash for fast comparison and copy
    hashed_ident: u64,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct OperatorIdentifier {
    op: Identifier,
    left: Identifier,
    right: Identifier,
}

pub fn reset_poison() {
    match BACKWARDS_MAP.lock() {
        Ok(_) => {}
        Err(_) => BACKWARDS_MAP.clear_poison()
    }
}

impl Identifier {
    pub fn new(ident: &str) -> Self {
        let mut hasher = DefaultHasher::new();

        ident.hash(&mut hasher);

        let hashed_ident = hasher.finish();


        BACKWARDS_MAP.lock().unwrap()
                     .entry(hashed_ident)
                     .or_insert_with(|| ident.to_string());

        Self { hashed_ident }
    }
}

impl OperatorIdentifier {
    pub fn new(op: Identifier, left: Identifier, right: Identifier) -> Self {
        Self { op, left, right }
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BACKWARDS_MAP.lock().unwrap().get(&self.hashed_ident).unwrap()
        )
    }
}

impl Debug for OperatorIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operator({} {} {})", self.left, self.op, self.right)
    }
}

impl Identifier {
    // builtin types
    pub fn for_nah() -> Self {
        Self::new("Nah")
    }

    pub fn for_number() -> Self {
        Self::new("Number")
    }

    pub fn for_bool() -> Self {
        Self::new("Bool")
    }

    pub fn for_string() -> Self {
        Self::new("String")
    }

    pub fn for_function() -> Self {
        Self::new("Function")
    }

    pub fn for_type() -> Self {
        Self::new("Type")
    }

    pub fn for_native_object() -> Self {
        Self::new("NativeObject")
    }

    // builtin operators
    pub fn for_plus() -> Self {
        Self::new("+")
    }

    pub fn for_minus() -> Self {
        Self::new("-")
    }

    pub fn for_multiply() -> Self {
        Self::new("*")
    }

    pub fn for_divide() -> Self {
        Self::new("/")
    }

    pub fn for_mod() -> Self {
        Self::new("%")
    }

    pub fn for_pow() -> Self {
        Self::new("**")
    }

    pub fn for_and() -> Self {
        Self::new("&&")
    }

    pub fn for_or() -> Self {
        Self::new("||")
    }

    pub fn for_combine() -> Self {
        Self::new("<>")
    }

    // builtin operators (comparison)
    pub fn for_less() -> Self {
        Self::new("<")
    }

    pub fn for_less_eq() -> Self {
        Self::new("<=")
    }

    pub fn for_greater() -> Self {
        Self::new(">")
    }

    pub fn for_greater_eq() -> Self {
        Self::new(">=")
    }

    pub fn for_eq() -> Self {
        Self::new("==")
    }

    pub fn for_not_eq() -> Self {
        Self::new("!=")
    }
}
