pub use crate::{
    control::{returned, returned_unit, Control},
    error::FruError,
    expression::FruExpression,
    fru_err_res,
    helpers::WrappingExtension,
    identifier::{Identifier, OperatorIdentifier},
    statement::FruStatement,
    static_native_value,
    thing::Thing,
    value::{
        builtin_function::BuiltinFunction,
        fru_function::FruFunction,
        fru_value::FruValue,
        function_helpers::{ArgumentList, EvaluatedArgumentList, FormalParameters},
        native_object::{cast_object, INativeObject, NativeObject},
        operator::Operator,
    },
};

pub use frugurt_macros::{derive_nat, static_ident};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct OfObject;
pub type IdOfObject = uid::Id<OfObject>;
