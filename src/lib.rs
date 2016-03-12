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

pub fn parse_line<T>(line : T) -> Option<i64> where T : AsRef<str>{
    let s = line.as_ref();
    if let nom::IResult::Done(left, res) = parser::line(s.as_bytes()) {
        if left.len() == 0 {
            Some(res)
        } else {
            None
        }
    } else {
        None
    }
}
