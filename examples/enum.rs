#![feature(specialization)]
use core::ops::Deref;

use heapless::Vec;
use layout_trait::*;

struct Proxy {}

#[derive(Debug)]
struct RegisterBlock {
    reg1: u32,
    reg2: u32,
}

impl Deref for Proxy {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        println!("--- Proxy deref ---");
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}

enum Enum {
    A(Proxy),
    B(u32, u32),
}

// emulate custom derive
impl GetLayoutType for Enum {
    fn get_layout_type<const N: usize>(layout: &mut Vec<Layout, N>) {
        Proxy::get_layout_type(layout);
        u32::get_layout_type(layout);
        u32::get_layout_type(layout);
    }
}

fn main() {
    //  let d = Enum::B(0);
    let d = Enum::A(Proxy {});
    let mut layout: Vec<Layout, 8> = Vec::new();
    d.get_layout(&mut layout);
    println!("{:?}", layout);
}
