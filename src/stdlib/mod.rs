pub mod builtins;
pub mod scope;

#[macro_export]
macro_rules! static_uid {
    () => {{
        static ID: once_cell::sync::Lazy<Id<OfObject>> = once_cell::sync::Lazy::new(Id::new);
        *ID
    }};
}

#[macro_export]
macro_rules! static_fru_value {
    ($t:tt) => {{
        static VALUE: once_cell::sync::Lazy<FruValue> =
            once_cell::sync::Lazy::new(|| NativeObject::new_value($t));
        VALUE.clone()
    }};
}
