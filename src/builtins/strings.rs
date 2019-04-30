use llvm_sys::prelude::*;
use llvm_sys::{core, memory_buffer};

pub fn initialize_get_string(&mut module: llvm_sys::LLVMModule) {
  let ret_type = unsafe { core::LLVMPointerType(LLVMInt32Type()) };
  
  let params_type = [].as_mut_ptr();
  
  let get_string_func_type = unsafe {
    core::LLVMFunctionType(ret_type, params_type, 0, 0)
  };
  
  // add the function to the module
  let get_string_func = unsafe { core::LLVMAddFunction(module, c_str!("getString")) };
}

pub fn initialize_put_string(&mut module: llvm_sys::LLVMModule) {
  let ret_type = unsafe { core::LLVMVoidType() };
}