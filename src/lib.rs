#![crate_name = "rust_i2p"]

extern crate chrono;
extern crate rand;
extern crate quickcheck;


pub mod common;
mod util;


#[cfg(test)]
#[path="tests/mod.rs"]
mod tests;
