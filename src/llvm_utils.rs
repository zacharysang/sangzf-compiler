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
use std::ffi::{CStr, CString};

use llvm_sys::core;
use llvm_sys::prelude::*;
use llvm_sys::core::*;

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

// return the llvm value corresponding to the given type and string
pub fn get_llvm_value(string: &str, t: &Type) -> Result<LLVMValueRef, String> {
  unsafe {
    return match t {
      Type::None => {
        Ok(LLVMConstInt(core::LLVMInt32Type(), 0, 0))
      },
      Type::Integer => {
        let val: u64 = string.parse().expect("Could not parse integer");
        Ok(LLVMConstInt(core::LLVMInt32Type(), val, 1))
      },
      Type::Float => {
        let val: f64 = string.parse().expect("Could not parse float");
        Ok(LLVMConstReal(core::LLVMFloatType(), val))
      },
      Type::Bool => {
        let val = if string == "true" {1} else {0};
        Ok(LLVMConstInt(core::LLVMInt32Type(), val, 0))
      },
      Type::String => {
        // TODO implement actual strings
        Ok(LLVMConstInt(core::LLVMInt32Type(), 0, 0))
      },
      _ => {
        // return 0 int as default value 
        // (consider returning a Result from this function and error here)
        // this represents an unsuccessful cast
        let mut err = String::from("Could not get llvm value for type '");
        err.push_str(&t.to_string());
        err.push_str("' from string: ");
        err.push_str(string);
        Err(err)
      }
    }
  };
}