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

use llvm_sys::core;
use llvm_sys::prelude::*;

use crate::tokenize::token::{Type};

// function to quickly create c strings from dynamic rust string slices
pub fn c_str(slice: &str) -> *const i8 {
  return CString::new(slice).expect("could not get c-string from slice").into_raw();
}

pub fn null_str() -> *const i8 {
    return c_str("");
}

pub fn error_buffer() -> *mut *mut i8 {
  let mut zero: [i8; 256];
  unsafe {
    zero = mem::zeroed();
  }
  return &mut zero.as_mut_ptr();
}

// return the llvm type based on the type
pub fn get_llvm_type(t: &Type) -> LLVMTypeRef {
  unsafe {
    return match t {
      Type::Integer => core::LLVMInt32Type(),
      Type::Float => core::LLVMFloatType(),
      Type::String => core::LLVMPointerType(core::LLVMInt32Type(), 0),
      Type::Bool =>  core::LLVMInt32Type(),
      Type::None => core::LLVMVoidType(),
      _ => {
        println!("type '{}' not supported yet", t.to_string());
        core::LLVMVoidType()
      }
    };
  }
}