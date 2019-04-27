use llvm_sys::prelude::*;
use llvm_sys::{core};

use crate::llvm_utils::{c_str};

pub fn initialize_bool_funcs(module: *mut llvm_sys::LLVMModule) -> (LLVMValueRef, LLVMValueRef){
  return (initialize_get_bool(module), initialize_put_bool(module));
}

fn initialize_get_bool(module: *mut llvm_sys::LLVMModule) -> LLVMValueRef {
  let ret_type = unsafe { core::LLVMInt32Type() };
  
  let params_type = [].as_mut_ptr();
  
  let get_bool_func_type = unsafe {
    core::LLVMFunctionType(ret_type, params_type, 0, 0)
  };
  
  let get_bool_func = unsafe {
    core::LLVMAddFunction(module, c_str("getBool"), get_bool_func_type)
  };
  
  return get_bool_func;
}

fn initialize_put_bool(module: *mut llvm_sys::LLVMModule) -> LLVMValueRef {
  let ret_type = unsafe { core::LLVMVoidType() };
  
  let params_type = unsafe {[core::LLVMInt32Type()].as_mut_ptr()};
  
  let put_bool_func_type = unsafe {
    core::LLVMFunctionType(ret_type, params_type, 1, 0)
  };
  
  let put_bool_func = unsafe {
    core::LLVMAddFunction(module, c_str("putBool"), put_bool_func_type)
  };
  
  return put_bool_func;
}