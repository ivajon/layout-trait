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

struct Custom {
    proxy: Proxy,
    b: u32,
}

// emulate custom derive
impl GetLayout for Custom {
    fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
        self.proxy.get_layout(layout);
        self.b.get_layout(layout);
    }
}

fn main() {
    let d = Custom {
        proxy: Proxy {},
        b: 0,
    };
    let mut layout: Vec<Layout, 8> = Vec::new();
    d.get_layout(&mut layout);
    println!("{:?}", layout);
}
