/*
// macro to quickly create c strings from rust string literals
// source: https://gist.githubusercontent.com/jayphelps/ee06dad051eb30d10982535958ad059a/raw/eeb9a68eb0fb9653a4b3af40dee75c4558d43238/main.rs
// usage: c_str!("hello, world.")
#[macro_export]
macro_rules! c_str {
    ($s:expr) => (
        // take s and add a null-termination and get as ptr encoded in utf8
        concat!($s, "\0").as_ptr() as *const i8
    );
}
*/

use std::mem;
use std::u64;
use std::ffi::{CStr, CString};

// function to quickly create c strings from dynamic rust string slices
pub fn c_str(slice: &str) -> *const i8 {
  return CString::new(slice).expect("could not get c-string from slice").into_raw();
}

pub fn null_str() -> *const i8 {
    return c_str("");
}

pub fn get_true() -> u64 {
  return u64::MAX;
}

pub fn get_false() -> u64 {
  return 0;
}

pub fn error_buffer() -> *mut *mut i8 {
  let mut zero: [i8; 256];
  unsafe {
    zero = mem::zeroed();
  }
  return &mut zero.as_mut_ptr();
}