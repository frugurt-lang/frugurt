use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{DefaultHasher, Hash, Hasher},
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

impl Identifier {
    pub fn new(ident: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        ident.hash(&mut hasher);
        let hashed_ident = hasher.finish();

        BACKWARDS_MAP
            .lock()
            .unwrap()
            .entry(hashed_ident)
            .or_insert_with(|| ident.to_string());

        Self { hashed_ident }
    }

    pub const fn new_unchecked(hashed_ident: u64) -> Self {
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
        write!(
            f,
            "{}",
            BACKWARDS_MAP.lock().unwrap().get(&self.hashed_ident).unwrap()
        )
    }
}

impl Debug for OperatorIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operator({} {} {})", self.left, self.op, self.right)
    }
}

pub mod id {
    use macros::static_ident;

    use crate::interpreter::identifier::Identifier;

    // types
    pub const NAH: Identifier = static_ident!("Nah");
    pub const NUMBER: Identifier = static_ident!("Number");
    pub const BOOL: Identifier = static_ident!("Bool");
    pub const STRING: Identifier = static_ident!("String");
    pub const FUNCTION: Identifier = static_ident!("Function");
    pub const TYPE: Identifier = static_ident!("Type");
    pub const NATIVE_OBJECT: Identifier = static_ident!("NativeObject");

    // arithmetic
    pub const PLUS: Identifier = static_ident!("+");
    pub const MINUS: Identifier = static_ident!("-");
    pub const MULTIPLY: Identifier = static_ident!("*");
    pub const DIVIDE: Identifier = static_ident!("/");
    pub const MOD: Identifier = static_ident!("%");
    pub const POW: Identifier = static_ident!("**");
    pub const AND: Identifier = static_ident!("&&");
    pub const OR: Identifier = static_ident!("||");
    pub const COMBINE: Identifier = static_ident!("<>");

    // comparison
    pub const LESS: Identifier = static_ident!("<");
    pub const LESS_EQ: Identifier = static_ident!("<=");
    pub const GREATER: Identifier = static_ident!(">");
    pub const GREATER_EQ: Identifier = static_ident!(">=");
    pub const EQ: Identifier = static_ident!("==");
    pub const NOT_EQ: Identifier = static_ident!("!=");
}
