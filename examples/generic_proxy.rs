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

struct Generic<T> {
    generic: T,
}

impl<T> layout_trait::GetLayout for Generic<T> {
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        self.generic.get_layout(f);
    }
}

impl<T> layout_trait::GetLayoutType for Generic<T> {
    fn get_layout_type<F: FnMut(usize, usize)>(f: &mut F) {
        T::get_layout_type(f);
    }
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Generic { generic: Proxy1 {} };

    let mut callback = |address, size| layout.push(Layout { address, size }).expect("Valid size");
    a.get_layout(&mut callback);
    println!("{:#x?}", layout);
}
