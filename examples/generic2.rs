#![feature(min_specialization)]

use heapless::Vec;
use layout_trait::*;

struct Generic<T1, T2> {
    generic1: T1,
    generic2: T2,
}

impl<T1, T2> layout_trait::GetLayout for Generic<T1, T2> {
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        self.generic2.get_layout(f);
        self.generic1.get_layout(f);
    }
}

impl<T1, T2> layout_trait::GetLayoutType for Generic<T1, T2> {
    fn get_layout_type<F: FnMut(usize, usize)>(f: &mut F) {
        T1::get_layout_type(f);
        T2::get_layout_type(f);
    }
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Generic {
        generic1: 0u32,
        generic2: 1u64,
    };

    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    a.get_layout(&mut callback);
    println!("{:#x?}", layout);
}
