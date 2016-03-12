#![feature(collections)]
#![feature(slice_patterns)]
#![no_std]

#![allow(unused_imports)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate collections;

pub mod parser;

mod std {
    pub use core::{fmt, iter, option, ops, slice, mem};
    pub use collections::{boxed, vec, string};
    pub mod prelude {
        pub use core::prelude as v1;
    }
}
