#[macro_export]
macro_rules! rc_refcell {
    ($val:expr) => {
        Rc::new(RefCell::new($val))
    };
}
