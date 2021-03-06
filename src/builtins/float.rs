use llvm_sys::prelude::*;
use llvm_sys::{core};

use crate::llvm_utils::{c_str};

use crate::tokenize::token::{Token, TokenEntry, Type};
use crate::tokens;

pub fn initialize_float_funcs(module: *mut llvm_sys::LLVMModule) -> (TokenEntry, TokenEntry){
  return (initialize_get_float(module), initialize_put_float(module));
}

fn initialize_get_float(module: *mut llvm_sys::LLVMModule) -> TokenEntry {

  let name = "getfloat";

  let ret_type = unsafe { core::LLVMFloatType() };
  let params_type = [].as_mut_ptr();
  let get_float_func_type = unsafe {
    core::LLVMFunctionType(ret_type, params_type, 0, 0)
  };
  
  let get_float_func = unsafe {
    core::LLVMAddFunction(module, c_str(name), get_float_func_type)
  };
  
  return TokenEntry{
    chars: String::from(name),
    tok_type: Token::Unknown(tokens::unknown::Unknown{state: None}), // none since this didn't come from the lexer
    line_num: 0,
    r#type: Type::Procedure(vec![], Box::new(Type::Float)),
    value_ref: get_float_func
  };
}

fn initialize_put_float(module: *mut llvm_sys::LLVMModule) -> TokenEntry {

  let name = "putfloat";

  let ret_type = unsafe { core::LLVMInt32Type() };
  let params_type = unsafe {[core::LLVMFloatType()].as_mut_ptr()};
  let put_float_func_type = unsafe {
    core::LLVMFunctionType(ret_type, params_type, 1, 0)
  };
  
  let put_float_func = unsafe {
    core::LLVMAddFunction(module, c_str(name), put_float_func_type)
  };
  
  return TokenEntry{
    chars: String::from(name),
    tok_type: Token::Unknown(tokens::unknown::Unknown{state: None}), // since not from lexer
    line_num: 0,
    r#type: Type::Procedure(vec![Box::new(Type::Float)], Box::new(Type::Bool)),
    value_ref: put_float_func
  };
}