//! # compile-rand
//!
//! This crate provides the `compile_rand!` macro, which will generate a random number of the given
//! integer type at compile time.
//!
//! In other words, if – for whatever reason – you actually need [this XKCD comic][221]:
//!
//! [![<https://xkcd.com/221/>](https://imgs.xkcd.com/comics/random_number.png)][221]
//!
//! ...then this crate will give it to you!
//!
//! ## Usage
//!
//! ```
//! use compile_rand::compile_rand;
//!
//! let r = compile_rand!(i64);
//! println!("{}", r); // A new number each time the program is compiled.
//! ```
//!
//! [221]: https://xkcd.com/221/

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn compile_rand(tokens: TokenStream) -> TokenStream {
  macro_rules! random {
    ($t:ty) => {{
      let r = rand::random::<$t>();
      quote! { #r }
    }};
  }
  match tokens.to_string().as_str() {
    "i8" => random!(i8),
    "i16" => random!(i16),
    "i32" => random!(i32),
    "i64" => random!(i64),
    "i128" => random!(i128),
    "u8" => random!(u8),
    "u16" => random!(u16),
    "u32" => random!(u32),
    "u64" => random!(u64),
    "u128" => random!(u128),
    _ => quote! { compile_error!("compile_rand must receive a Rust fixed integer type") },
  }
  .into()
}
