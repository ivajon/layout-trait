#![feature(min_specialization)]
use core::ops::Deref;

#[derive(Debug)]
pub struct Layout {
    pub address: usize,
    pub size: usize,
}
pub trait SafeLayout {
    fn get_layout(&self) -> Layout;
}

impl<T> SafeLayout for T {
    default fn get_layout(&self) -> Layout {
        Layout {
            #[allow(suspicious_double_ref_op)]
            address: self.deref() as *const _ as usize,
            size: core::mem::size_of_val(self.deref()),
        }
    }
}
