#![crate_name = "rusti2p"]
extern crate chrono;
extern crate rand;
extern crate quickcheck;
extern crate rustc_serialize;


pub mod common;
mod serialize;


#[cfg(test)]
#[path="tests/lib.rs"]
mod tests;
