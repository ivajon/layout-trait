#![cfg_attr(not(test), no_std)]
#![feature(specialization)]
#![allow(incomplete_features)]
use core::ops::Deref;

#[cfg(feature = "derive")]
pub use layout_derive::Layout as DeriveLayout;

#[derive(Debug, PartialEq, Eq)]
pub struct Layout {
    pub address: usize,
    pub size: usize,
}

pub trait GetLayout {
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F);
}

impl<T> GetLayout for T {
    default fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        f(self as *const _ as usize, core::mem::size_of_val(self))
    }
}

impl<T, U: GetLayout> GetLayout for T
where
    T: Deref<Target = U>,
{
    fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
        let data = self.deref();

        f(data as *const _ as usize, core::mem::size_of_val(data))
    }
}

pub trait GetLayoutType {
    fn get_layout_type<F: FnMut(usize, usize)>(f: &mut F);
}

impl<T> GetLayoutType for T {
    default fn get_layout_type<F: FnMut(usize, usize)>(_f: &mut F) {
        // by default this does nothing
        // we override this for enums/unions
    }
}

impl<T, U> GetLayoutType for T
where
    // for now assume this to ZST peripheral proxy
    T: Deref<Target = U>,
{
    default fn get_layout_type<F: FnMut(usize, usize)>(f: &mut F) {
        // hopefully there is a better way to do this
        // for now we crate a &ZST out of thin air!!!
        let t: &T = unsafe { core::mem::transmute(&()) };
        let data = t.deref();

        f(data as *const _ as usize, core::mem::size_of_val(data))
    }
}

#[cfg(test)]
mod test {
    use core::{cell::UnsafeCell, mem::MaybeUninit};

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
        let layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4)
    }

    #[test]
    fn test_array_u32() {
        let data: [u32; 16] = [32; 16];

        let mut layout: Vec<Layout, 8> = Vec::new();
        let layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 64)
    }

    struct Simple {
        data: u32,
        data2: u64,
    }

    // this implementation should be generated by a custom derive
    impl GetLayout for Simple {
        fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
            self.data.get_layout(f);
            self.data2.get_layout(f);
        }
    }

    #[test]
    fn test_simple() {
        let data = Simple { data: 0, data2: 0 };
        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
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
        fn get_layout<F: FnMut(usize, usize)>(&self, f: &mut F) {
            self.simple.get_layout(f);
            self.data2.get_layout(f);
        }
    }

    #[test]
    fn test_complex() {
        let data = Complex {
            simple: Simple { data: 0, data2: 0 },
            data2: Proxy {},
        };

        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut callback = |ptr, size| {
            layout
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
        assert!(layout[2].size == 8); // Proxy size
        assert!(layout[2].address == 0x1000); // Proxy address
        let data = Complex {
            simple: Simple { data: 0, data2: 0 },
            data2: Proxy {},
        };
        let data = MaybeUninit::new(data);

        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut callback = |ptr, size| {
            layout
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
        assert!(layout[2].size == 8); // Proxy size
        assert!(layout[2].address == 0x1000); // Proxy address
        let data = UnsafeCell::new(MaybeUninit::new(data));

        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut callback = |ptr, size| {
            layout
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
        assert!(layout[2].size == 8); // Proxy size
        assert!(layout[2].address == 0x1000); // Proxy address
    }

    // #[test]
    // fn test_complex_indirect() {
    //     let data = Complex {
    //         simple: Simple { data: 0, data2: 0 },
    //         data2: Proxy {},
    //     };
    //     let data = UnsafeCell::new(MaybeUninit::new(data));
    //     let uninit: MaybeUninit<_> = unsafe { *(data.get() as *const _) };
    //
    //     let mut layout: Vec<Layout, 8> = Vec::new();
    //     let mut callback = |ptr, size| {
    //         layout
    //             .push(Layout { address: ptr, size })
    //             .expect("Propper capacity")
    //     };
    //     data.get_layout(&mut callback);
    //     println!("{:#x?}", layout);
    //
    //     assert!(layout[0].size == 4);
    //     assert!(layout[1].size == 8);
    //     assert!(layout[2].size == 8); // Proxy size
    //     assert!(layout[2].address == 0x1000); // Proxy address
    // }
    enum Enum {
        A,
        B(u32),
        C(Proxy),
    }

    // this implementation should be generated by a custom derive
    impl GetLayoutType for Enum {
        fn get_layout_type<F: FnMut(usize, usize)>(f: &mut F) {
            println!("--- GetLayoutType for Enum ---");
            u32::get_layout_type(f);
            Proxy::get_layout_type(f);
        }
    }

    #[test]
    fn test_enum() {
        // The A variant
        let mut data = Enum::A;

        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        data = Enum::B(1);
        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        // the B variant location
        if let Enum::B(ref x) = data {
            println!("Enum::B {} {}", x, x as *const _ as usize);
        }

        //
        data = Enum::C(Proxy {});
        let mut layout: Vec<Layout, 8> = Vec::new();
        let mut layout_ref = &mut layout;
        let mut callback = |ptr, size| {
            layout_ref
                .push(Layout { address: ptr, size })
                .expect("Propper capacity")
        };
        data.get_layout(&mut callback);
        println!("{:#x?}", layout);

        if let Enum::C(ref c) = data {
            println!("Enum::C {}", c as *const _ as usize);
        }
    }
}
