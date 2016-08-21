#![crate_name = "rusti2p"]
extern crate chrono;
extern crate rand;
extern crate quickcheck;


pub mod common;
mod util;


#[cfg(test)]
#[path="tests/lib.rs"]
mod tests;
