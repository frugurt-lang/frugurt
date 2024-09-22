pub mod helpers;
pub mod prelude;
pub mod common;

#[macro_export]
macro_rules! static_uid {
    () => {{
        static ID: once_cell::sync::Lazy<
            uid::Id<$crate::common::OfObject>,
        > = once_cell::sync::Lazy::new(uid::Id::new);
        *ID
    }};
}

#[macro_export]
macro_rules! static_native_value {
    ($t:tt) => {{
        static VALUE: once_cell::sync::Lazy<FruValue> = once_cell::sync::Lazy::new(|| {
            $crate::common::NativeObject::new_value($t)
        });
        VALUE.clone()
    }};
}

#[macro_export]
macro_rules! static_op1 {
    () => {
        static OPERATORS: once_cell::sync::Lazy<
            std::sync::Mutex<
                std::collections::HashMap<
                    $crate::common::OperatorIdentifier,
                    $crate::common::operator::AnyOperator,
                >,
            >,
        > = once_cell::sync::Lazy::new(Default::default);
    };
}
