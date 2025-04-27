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
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        self.0.get_layout(f);
        self.1.get_layout(f);
    }
}

impl layout_trait::GetLayoutType for Tuple {
    fn get_layout_type_callback<F: FnMut(usize, usize)>(f: &mut F) {
        u32::get_layout_type_callback(f);
        Proxy1::get_layout_type_callback(f);
    }
}
enum Enum {
    A(Tuple),
}

impl layout_trait::GetLayoutType for Enum {
    fn get_layout_type_callback<F: FnMut(usize, usize)>(f: &mut F) {
        Tuple::get_layout_type_callback(f);
    }
}

fn main() {
    let a = Enum::A(Tuple(0, Proxy1 {}));

    let mut layout: Vec<Layout, 8> = Vec::new();
    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    a.get_layout(&mut callback);
    println!("{:#x?}", layout);
}
