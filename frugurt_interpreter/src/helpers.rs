use std::rc::Rc;

pub trait WrappingExtension {
    fn wrap_box(self) -> Box<Self>;

    fn wrap_rc(self) -> Rc<Self>;
}

impl<T> WrappingExtension for T
where
    T: 'static + Sized,
{
    fn wrap_box(self) -> Box<Self> {
        Box::new(self)
    }

    fn wrap_rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}
