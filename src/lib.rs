#![crate_name = "gmp"]
#![warn(deprecated)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate num_traits;

mod ffi;
pub mod mpf;
pub mod mpq;
pub mod mpz;
pub mod rand;
pub mod sign;

#[cfg(test)]
mod test;
