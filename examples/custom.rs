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

struct Custom {
    proxy: Proxy,
    b: u32,
}

// emulate custom derive
impl GetLayout for Custom {
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        self.proxy.get_layout(f);
        self.b.get_layout(f);
    }
}

fn main() {
    let d = Custom {
        proxy: Proxy {},
        b: 0,
    };
    let mut layout: Vec<Layout, 8> = Vec::new();
    let mut callback = |ptr, size| {
        layout
            .push(Layout { address: ptr, size })
            .expect("Propper size")
    };
    d.get_layout(&mut callback);
    println!("{:#x?}", layout);
}
