#![feature(min_specialization)]
use core::ops::Deref;

use heapless::Vec;
use layout_trait::*;

struct Proxy {}

#[derive(Debug)]
struct RegisterBlock {
    _reg1: u32,
    _reg2: u32,
}

impl Deref for Proxy {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        // println!("--- Proxy deref ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

#[derive(DeriveLayout)]
enum Enum {
    A(Proxy),
    #[allow(unused)]
    B(u32, u32),
}

fn main() {
    let d = Enum::A(Proxy {});
    let mut layout: Vec<Layout, 8> = Vec::new();
    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    d.get_layout(&mut callback);
    println!("Variant A : {:#x?}", layout);
    let d = Enum::B(32, 1234);
    let mut layout: Vec<Layout, 8> = Vec::new();
    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    d.get_layout(&mut callback);
    println!("Variant B {:#x?}", layout);
}
