#![feature(min_specialization)]

use heapless::Vec;
use layout_trait::*;

struct Generic<T1, T2> {
    generic1: T1,
    generic2: T2,
}

impl<T1, T2> layout_trait::GetLayout for Generic<T1, T2> {
    fn get_layout<const N: usize>(
        &self,
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        self.generic1.get_layout(layout);
        self.generic2.get_layout(layout);
    }
}

impl<T1, T2> layout_trait::GetLayoutType for Generic<T1, T2> {
    fn get_layout_type<const N: usize>(
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        T1::get_layout_type(layout);
        T2::get_layout_type(layout);
    }
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Generic {
        generic1: 0u32,
        generic2: 1u64,
    };

    a.get_layout(&mut layout);
    println!("{:#x?}", layout);
}
