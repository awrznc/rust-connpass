extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
mod tests;

mod connpass;
pub use self::connpass::event;
