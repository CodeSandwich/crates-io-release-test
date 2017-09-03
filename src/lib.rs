#![feature(proc_macro)]
//! EXTERNAL LIB

#![doc(html_logo_url = "https://raw.githubusercontent.com/CodeSandwich/crates-io-release-test/master/test-logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/CodeSandwich/crates-io-release-test/master/test-logo.png")]

extern crate code_sandwich_crates_io_release_test_macros;

use code_sandwich_crates_io_release_test_macros::*;

pub mod macros {
    pub use code_sandwich_crates_io_release_test_macros::*;
}

#[my_macro]
pub fn main() {
    println!("Hello, world!");
}
