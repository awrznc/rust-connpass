//! # rust-connpass
//!
//! [Connpass API](https://connpass.com/about/api/) library in Rust.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
mod tests;

mod connpass;
pub use self::connpass::event;
