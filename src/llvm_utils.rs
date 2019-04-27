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

//use std::char;

// function to quickly create c strings from dynamic rust string slices
pub fn c_str(slice: &str) -> *const i8 {

  // code for adding null terminator to string
  let mut s = String::from(slice);
  //s.push(char::from(0 as u8));
  //for ch in b"\0" {
  //  s.push(char::from(*ch));
  //}
  
  return slice.as_ptr() as *const i8;
}

pub fn null_str() -> *const i8 {
    return b"\0".as_ptr() as *const _;
}