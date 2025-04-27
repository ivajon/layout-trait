#![feature(min_specialization)]

use heapless::Vec;
use layout_trait::*;

struct Generic<T> {
    generic: T,
}

impl<T> layout_trait::GetLayout for Generic<T> {
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        self.generic.get_layout(f);
    }
}

impl<T> layout_trait::GetLayoutType for Generic<T> {
    fn get_layout_type_callback<F: FnMut(usize, usize)>(f: &mut F) {
        T::get_layout_type_callback(f);
    }
}

fn main() {
    let a = Generic { generic: 0u32 };

    let mut layout: Vec<Layout, 8> = Vec::new();
    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    a.get_layout(&mut callback);
    println!("{:#x?}", layout);
}
