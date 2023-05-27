#![cfg_attr(not(test), no_std)]
#![feature(specialization)]
use core::ops::Deref;

pub use heapless;

#[derive(Debug, PartialEq, Eq)]
pub struct Layout {
    pub address: usize,
    pub size: usize,
}
pub trait GetLayout {
    fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>);
}

impl<T> GetLayout for T {
    default fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>) {
        // println!("--- default GetLayout for T ---");
        layout
            .push(Layout {
                address: self as *const _ as usize,
                size: core::mem::size_of_val(self),
            })
            .unwrap();

        // println!("T::");
        T::get_layout_type(layout);
    }
}

impl<T, U> GetLayout for T
where
    T: Deref<Target = U>,
{
    fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>) {
        // println!("--- deref GetLayout for T ---");
        let data = self.deref();
        layout
            .push(Layout {
                address: data as *const _ as usize,
                size: core::mem::size_of_val(data),
            })
            .unwrap();
    }
}

pub trait GetLayoutType {
    fn get_layout_type<const N: usize>(layout: &mut heapless::Vec<Layout, N>);
}

impl<T> GetLayoutType for T {
    // by default this does nothing
    // we override this for enums/unions
    default fn get_layout_type<const N: usize>(_layout: &mut heapless::Vec<Layout, N>) {
        // println!("--- default GetLayoutType for T ---");
    }
}

impl<T, U> GetLayoutType for T
where
    // for now assume this to ZST peripheral proxy
    T: Deref<Target = U>,
{
    default fn get_layout_type<const N: usize>(layout: &mut heapless::Vec<Layout, N>) {
        // println!("--- deref GetLayoutType for T ---");

        // hopefully there is a better way to do this
        // for now we crate a &ZST out of thin air!!!
        let t: &T = unsafe { core::mem::transmute(&()) };
        let data = t.deref();
        layout
            .push(Layout {
                address: data as *const _ as usize,
                size: core::mem::size_of_val(data),
            })
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use heapless::Vec;

    // emulate the svd2rust peripheral proxy

    #[derive(Debug)]
    struct RegisterBlock {
        _reg1: u32,
        _reg2: u32,
    }

    struct Proxy {}

    impl Deref for Proxy {
        type Target = RegisterBlock;
        fn deref(&self) -> &Self::Target {
            unsafe { &*(0x1000 as *const RegisterBlock) }
        }
    }

    #[test]
    fn test_u32() {
        let data: u32 = 32;
        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4)
    }

    #[test]
    fn test_array_u32() {
        let data: [u32; 16] = [32; 16];

        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 64)
    }

    struct Simple {
        data: u32,
        data2: u64,
    }

    // this implementation should be generated by a custom derive
    impl GetLayout for Simple {
        fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
            // get_layout is executed on each field
            self.data.get_layout(layout);
            self.data2.get_layout(layout);
        }
    }

    #[test]
    fn test_simple() {
        let data = Simple { data: 0, data2: 0 };
        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
    }

    struct Complex {
        simple: Simple,
        data2: Proxy,
    }

    // this implementation should be generated by a custom derive
    impl GetLayout for Complex {
        fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
            // get_layout is executed on each field
            self.simple.get_layout(layout);
            self.data2.get_layout(layout);
        }
    }

    #[test]
    fn test_complex() {
        let data = Complex {
            simple: Simple { data: 0, data2: 0 },
            data2: Proxy {},
        };

        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
        assert!(layout[2].size == 8); // Proxy size
        assert!(layout[2].address == 0x1000); // Proxy address
    }
    enum Enum {
        A,
        B(u32),
        C(Proxy),
    }

    // this implementation should be generated by a custom derive
    impl GetLayoutType for Enum {
        fn get_layout_type<const N: usize>(layout: &mut Vec<Layout, N>) {
            println!("--- GetLayoutType for Enum ---");
            u32::get_layout_type(layout);
            Proxy::get_layout_type(layout);
        }
    }

    #[test]
    fn test_enum() {
        // The A variant
        let mut data = Enum::A;

        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        let mut layout: Vec<Layout, 8> = Vec::new();
        data = Enum::B(1);
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        // the B variant location
        if let Enum::B(ref x) = data {
            println!("Enum::B {} {}", x, x as *const _ as usize);
        }

        //
        let mut layout: Vec<Layout, 8> = Vec::new();
        data = Enum::C(Proxy {});
        data.get_layout(&mut layout);
        println!("{:#x?}", layout);

        if let Enum::C(ref c) = data {
            println!("Enum::C {}", c as *const _ as usize);
        }
    }
}
