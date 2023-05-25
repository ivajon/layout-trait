#![feature(specialization)]
use core::ops::Deref;

pub use heapless;

#[derive(Debug)]
pub struct Layout {
    pub address: usize,
    pub size: usize,
}
pub trait GetLayout {
    fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>);
}

impl<T> GetLayout for T {
    default fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>) {
        layout
            .push(Layout {
                address: self.deref() as *const _ as usize,
                size: core::mem::size_of_val(self.deref()),
            })
            .unwrap();
    }
}

impl<T, U> GetLayout for T
where
    T: Deref<Target = U>,
{
    default fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>) {
        println!("--- Deref -- ");
        layout
            .push(Layout {
                address: self.deref() as *const _ as usize,
                size: core::mem::size_of_val(self.deref()),
            })
            .unwrap();
    }
}

struct Proxy {}

#[derive(Debug)]
struct RegisterBlock {
    reg1: u64,
}

impl Deref for Proxy {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(0x1000 as *const RegisterBlock) }
    }
}
use heapless::Vec;
fn main() {
    let d = Proxy {};

    // println!("deref {:?}", *d);
    println!("---");

    let mut layout: Vec<Layout, 8> = Vec::new();
    d.get_layout(&mut layout);
    println!("{:?}", layout);

    let d = 0u32;
    let mut layout: Vec<Layout, 8> = Vec::new();
    d.get_layout(&mut layout);
    println!("{:?}", layout);
}
