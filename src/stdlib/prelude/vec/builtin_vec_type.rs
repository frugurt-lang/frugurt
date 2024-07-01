use std::{fmt::Debug, rc::Rc};

use frugurt_macros::derive_nat;

use crate::{
    fru_err_res,
    interpreter::{
        error::FruError,
        value::{
            fru_value::FruValue, function_helpers::EvaluatedArgumentList,
            native_object::INativeObject, native_object::NativeObject,
        },
    },
    static_native_value,
    stdlib::prelude::vec::builtin_vec_instance::BuiltinVecInstance,
};

pub struct BuiltinVecType;

impl BuiltinVecType {
    pub fn get_singleton() -> FruValue {
        static_native_value!(BuiltinVecType)
    }
}

#[derive_nat(as_any, get_uid, get_type, get_set_op, fru_clone)]
impl INativeObject for BuiltinVecType {
    // fn get_prop(self: Rc<Self>, ident: Identifier) -> Result<FruValue, FruError> {}

    fn instantiate(self: Rc<Self>, mut args: EvaluatedArgumentList) -> Result<FruValue, FruError> {
        let args = args
            .args
            .drain(..)
            .map(|(ident, v)| {
                if let Some(ident) = ident {
                    fru_err_res!("vector item `{}` can not be named", ident)
                } else {
                    Ok(v)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(NativeObject::new_value_rc(BuiltinVecInstance::new(args)))
    }
}

impl Debug for BuiltinVecType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec")
    }
}
