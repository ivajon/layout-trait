#![feature(min_specialization)]
use core::ops::Deref;
use heapless::Vec;
use layout_trait::*;

struct Proxy1 {}

#[derive(Debug)]
struct RegisterBlock {
    _reg1: u32,
    _reg2: u32,
}

impl Deref for Proxy1 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref 1 ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

struct Tuple(u32, Proxy1);

impl layout_trait::GetLayout for Tuple {
    fn get_layout<const N: usize>(
        &self,
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        self.0.get_layout(layout);
        self.1.get_layout(layout);
    }

    fn get_layout_callback<F: Fn(usize, usize)>(&self, f: &F) {
        self.0.get_layout_callback(f);
        self.1.get_layout_callback(f);
    }
}

impl layout_trait::GetLayoutType for Tuple {
    fn get_layout_type<const N: usize>(
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        u32::get_layout_type(layout);
        Proxy1::get_layout_type(layout);
    }
    fn get_layout_type_callback<F: Fn(usize, usize)>(f: &F) {
        u32::get_layout_type_callback(f);
        Proxy1::get_layout_type_callback(f);
    }
}
enum Enum {
    A(Tuple),
}

impl layout_trait::GetLayoutType for Enum {
    fn get_layout_type<const N: usize>(
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        Tuple::get_layout_type(layout);
    }

    fn get_layout_type_callback<F: Fn(usize, usize)>(f: &F) {
        Tuple::get_layout_type_callback(f);
    }
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Enum::A(Tuple(0, Proxy1 {}));

    a.get_layout(&mut layout);
    println!("{:#x?}", layout);
}
