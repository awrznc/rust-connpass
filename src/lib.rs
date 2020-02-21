//! # rust-connpass
//!
//! [Connpass API](https://connpass.com/about/api/) library in Rust.
//!
//! ## Install
//! 
//! Install openssl through your favourite package. You might also need a C compiler (gcc).
//! 
//! ```bash
//! # Ubuntu, Alpine 
//! apt-get install libssl-dev pkg-config
//! 
//! # Arch Linux
//! pacman -S openssl
//! 
//! # Fedora, CentOS8
//! dnf install openssl-devel
//! 
//! # Amazon Linux
//! yum install openssl-devel
//! ```
//! 
//! ## Get Started
//! 
//! Put the following in your project's Cargo.toml file:
//! 
//! ```toml
//! [dependencies]
//! connpass = "0.1.0"
//! ```
//! 
//! And overwrite in your project's main.rs file:
//! 
//! ```rust
//! fn main() {
//!     // Declare query params
//!     let mut query_params = [("keyword", "Rust")];
//! 
//!     // Get event information
//!     let response = connpass::event::new().query(&mut query_params).get().expect("request error.");
//! 
//!     // Print event information
//!     let event = &response.body.unwrap().events[0];
//!     println!("event title ... {}", event.title);
//!
//!     // output example:
//!     // event title ... Rust入門者向けハンズオン 
//! }
//! ```
//! 
//! The event title is outputted when run build on the console.
//! 
//! ## Example
//! 
//! ```bash
//! cargo run --example get
//! ```
//! 
//! ## Contribute
//! 
//! Any PR is welcomed!
//! 
//! ## License
//! 
//! https://github.com/awrznc/rust-connpass/blob/master/LICENSE

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
mod tests;

mod connpass;
pub use self::connpass::event;
