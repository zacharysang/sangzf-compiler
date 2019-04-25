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

// function to quickly create c strings from dynamic rust string slices
pub fn c_str(slice: &str) -> *const i8 {
  return slice.as_ptr() as *const i8
}