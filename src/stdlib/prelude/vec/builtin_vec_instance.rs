use std::{
    cell::{OnceCell, RefCell},
    fmt::Debug,
    rc::Rc,
};

use uid::Id;

use frugurt_macros::{derive_nat, static_ident};

use crate::{
    fru_err_res,
    interpreter::{
        error::FruError,
        identifier::Identifier,
        value::{
            fru_value::FruValue,
            function_helpers::EvaluatedArgumentList,
            native_object::{INativeObject, NativeObject, OfObject},
        },
    },
    stdlib::{
        helpers::simple_method_of::SimpleMethodOf, prelude::vec::builtin_vec_type::BuiltinVecType,
    },
};

pub struct BuiltinVecInstance {
    value: RefCell<Vec<FruValue>>,
    uid: Id<OfObject>,
    method_push: OnceCell<FruValue>,
    at_method: OnceCell<FruValue>,
}

impl BuiltinVecInstance {
    pub fn new(value: Vec<FruValue>) -> Rc<BuiltinVecInstance> {
        let o = Rc::new(BuiltinVecInstance {
            value: RefCell::new(value),
            uid: Id::new(),
            method_push: OnceCell::new(),
            at_method: OnceCell::new(),
        });

        o.method_push
            .set(NativeObject::new_value(SimpleMethodOf::new(
                static_ident!("Push"),
                o.clone(),
                method_push,
            )))
            .unwrap();

        o.at_method
            .set(NativeObject::new_value(SimpleMethodOf::new(
                static_ident!("At"),
                o.clone(),
                method_at,
            )))
            .unwrap();

        o
    }
}

#[derive_nat(as_any, fru_clone)]
impl INativeObject for BuiltinVecInstance {
    fn get_uid(&self) -> Id<OfObject> {
        self.uid
    }

    fn get_type(&self) -> FruValue {
        BuiltinVecType::get_singleton()
    }

    fn get_prop(self: Rc<Self>, ident: Identifier) -> Result<FruValue, FruError> {
        if ident == static_ident!("Length") {
            Ok(FruValue::Number(self.value.borrow().len() as f64))
        } else if ident == static_ident!("Push") {
            Ok(self.method_push.get().unwrap().clone())
        } else if ident == static_ident!("At") {
            Ok(self.at_method.get().unwrap().clone())
        } else {
            fru_err_res!("`{:?}` has no prop `{}`", self.get_type(), ident)
        }
    }
}

impl Debug for BuiltinVecInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.value.borrow())
    }
}

// methods

fn method_push(
    this: &Rc<BuiltinVecInstance>,
    mut args: EvaluatedArgumentList,
) -> Result<FruValue, FruError> {
    if args.args.len() != 1 {
        return fru_err_res!("`{:?}` takes exactly one argument", this.get_type());
    }

    this.value.borrow_mut().push(args.args.drain(..).next().unwrap().1);

    Ok(FruValue::Number(this.value.borrow().len() as f64 - 1.))
}

fn method_at(
    this: &Rc<BuiltinVecInstance>,
    mut args: EvaluatedArgumentList,
) -> Result<FruValue, FruError> {
    if args.args.len() != 1 {
        return fru_err_res!("`{:?}` takes exactly one argument", this.get_type());
    }

    let index = args.args.drain(..).next().unwrap().1;

    let index = match index {
        FruValue::Number(index) => index,
        _ => return fru_err_res!("`{:?}` is not a number", index),
    };

    if index.fract() != 0. {
        return fru_err_res!("index must be an integer, not `{:?}`", index);
    }

    let index = index as usize;

    if index >= this.value.borrow().len() {
        return fru_err_res!("index out of bounds");
    }

    Ok(this.value.borrow().get(index).unwrap().clone())
}
