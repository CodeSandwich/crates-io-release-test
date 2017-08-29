#![feature(proc_macro)]

//! INTERNAL LIB [external lib](https://docs.rs/code-sandwich-crates-io-release-test/newest/code_sandwich_crates_io_release_test/fn.main.html)
//! [another](fn.my_macro.html)
extern crate proc_macro;
extern crate syn;
extern crate quote;


use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::str::FromStr;

#[proc_macro_attribute]
pub fn my_macro(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    token_stream
}

