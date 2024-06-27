pub mod builtins;

#[macro_export]
macro_rules! static_uid {
    () => {{
        static ID: once_cell::sync::Lazy<
            uid::Id<$crate::interpreter::value::native_object::OfObject>,
        > = once_cell::sync::Lazy::new(uid::Id::new);
        *ID
    }};
}

#[macro_export]
macro_rules! static_native_value {
    ($t:tt) => {{
        static VALUE: once_cell::sync::Lazy<FruValue> =
            once_cell::sync::Lazy::new(|| NativeObject::new_value($t));
        VALUE.clone()
    }};
}
