#![feature(proc_macro)]
//! EXTERNAL LIB

extern crate code_sandwich_crates_io_release_test_macros;

use code_sandwich_crates_io_release_test_macros::*;

pub mod macros {
    pub use code_sandwich_crates_io_release_test_macros::*;
}

#[my_macro]
pub fn main() {
    println!("Hello, world!");
}
