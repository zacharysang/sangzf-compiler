extern crate llvm_sys;

// import llvm dependencies
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::{core, bit_writer, analysis, support};

// llvm references used as guides
// * introduction to building llvm program using c-apis: https://pauladamsmith.com/blog/2015/01/how-to-get-started-with-llvm-c-api.html
// * using llvm with rust (+ webassembly, but I didn't use that part): https://medium.com/@jayphelps/using-llvm-from-rust-to-generate-webassembly-93e8c193fdb4
// * walkthrough on a compiler with llvm: https://releases.llvm.org/1.1/docs/Stacker.html#terminate

use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::slice::Iter;

use crate::tokenize::lexable::Lexable;
use crate::lexer::Lexer;
use crate::tokenize::token::Token;
use crate::tokenize::token::TokenEntry;
use crate::tokenize::token::Type;

use crate::builtins;
use crate::llvm_utils::{c_str, null_str, error_buffer, get_llvm_type};

use crate::tokens;


pub struct Parser<'a> {
  pub lexer: Peekable<Lexer<'a>>,
  pub symbol_table_chain: Vec<HashMap<String, Rc<TokenEntry>>>,
  pub global_symbol_table: HashMap<String, Rc<TokenEntry>>,
  pub llvm_module: *mut llvm_sys::LLVMModule
}

impl <'a>Parser<'a> {
  pub fn new(program: Peekable<Chars<'a>>) -> Self {
  
    let lexer = Lexer::new(program);
    
    let symbol_table_chain = vec![];
    
    let parser = Parser {
      lexer: lexer.peekable(),
      symbol_table_chain: symbol_table_chain,
      global_symbol_table: HashMap::new(),
      llvm_module: unsafe { core::LLVMModuleCreateWithName(c_str("compiler_module")) }
    };
    
    return parser;
  }
  
  
  /* 
    Below are functions for different parse rules
    Each function operates on an instance of the parser, processes the parse rule, and returns the success state or the reason why the parse failed
    On a failed parse, an unknown number of tokens will be consumed (may have partially consumed)
    For 'parse_tok', if unsuccessful, no tokens will have been consumed
  */
  
  pub fn program(&mut self) -> ParserResult {
  
    // create a global symbol table
    self.symbol_table_chain.push(HashMap::new());
    
    // set up the built-in functions
    // TODO make this actually work so manually linking isn't required
    /*
    unsafe {
      if support::LLVMLoadLibraryPermanently(c_str("./src/builtins/builtins.so")) == 0 {
        println!("load library: true");
      } else {
        println!("load library: false");
      }
    }
    */
    
    let program_header = self.program_header();
    if let ParserResult::Success(identifier_entry) = program_header {
    
      // create type for program
      let program_ret_type = unsafe { core::LLVMVoidType() };
      
      // create arguments type for program
      let program_param_types = unsafe {
        let param_types = [core::LLVMInt32Type(), core::LLVMInt32Type()].as_mut_ptr();
        param_types
      };
      
      // create function type
      let program_type = unsafe { core::LLVMFunctionType(program_ret_type, program_param_types, 0, 0) };
      
      // create main program function (type: core::LLVMValueRef)
      let program = unsafe { core::LLVMAddFunction(self.llvm_module, c_str("main"), program_type) };
      
      // add basic block to the program function (type: core::LLVMBasicBlockRef)
      let entry = unsafe { core::LLVMAppendBasicBlock(program, c_str("entry")) };
      
      // create builder and position it at the end of basic block (type: LLVMBuilderRef)
      let mut builder = unsafe { 
        let b = core::LLVMCreateBuilder();
        core::LLVMPositionBuilderAtEnd(b, entry);
        
        b
      };
      
      
      self.add_builtins(&mut builder);
    
      let program_body = self.program_body(&mut builder);
      if let ParserResult::Success(_) = program_body {
      
        // build the return
        unsafe {
          core::LLVMBuildRetVoid(builder);
        }
      
        // verify the module
        unsafe {
          let mut error: *mut *mut i8 = error_buffer();
          analysis::LLVMVerifyModule(self.llvm_module, analysis::LLVMVerifierFailureAction::LLVMAbortProcessAction, error);
          
          core::LLVMDisposeMessage(*error);
        }
      
        let mut filename = identifier_entry.chars;
        filename.push_str(".bc");
      
        // output contents of llvm program
        unsafe {
          if bit_writer::LLVMWriteBitcodeToFile(self.llvm_module, c_str(&filename)) != 0 {
            println!("error writing bitcode to file, skipping\n");
          }
        }
        
        // clean up builder, module, and context
        unsafe {
          core::LLVMDisposeModule(self.llvm_module);
          core::LLVMDisposeBuilder(builder);
        }
      
        // Check for terminating period
        let period = self.parse_tok(tokens::period::Period::start());
        if let ParserResult::Success(period_entry) = period {
          
          if let Some(tok_entry) = self.lexer.peek() {
            // unexpected token after end of program
            let result = ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num, expected: String::from("<end of program>"), actual: String::from(&tok_entry.chars[..])};
            
            result.print();
            
            return result;
          } else {
            if let Some(global_table) = self.symbol_table_chain.pop() {
              Parser::print_symbol_table(String::from("Global table"), &global_table);
            }
            println!("Program parsed.");
            return ParserResult::Success(period_entry);
          }
        } else { period.print(); return period; }
      } else { program_body.print(); return program_body; }
    } else { program_header.print(); return program_header; }
    
  }
  
  pub fn program_header(&mut self) -> ParserResult {
    // tok_entry is a borrowed value so it will not be able to be moved
    let program_kw = self.parse_tok(tokens::program_kw::ProgramKW::start());
    if let ParserResult::Success(..) = program_kw {
    
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success(identifier_entry) = identifier {
        let is_kw = self.parse_tok(tokens::is_kw::IsKW::start());
        if let ParserResult::Success(tok_entry) = &is_kw {
          return ParserResult::Success(identifier_entry);
        } else {
          is_kw.print();
          return is_kw;
        }
        
      } else { identifier.print(); return identifier; }
    } else { program_kw.print(); return program_kw; }
  }
  
  pub fn program_body(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
    
    // create a new symbol table for this scope
    self.symbol_table_chain.push(HashMap::new());
    
    // while next token is in First(declaration), read in a declaration w/ semicolon terminator 
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        match &tok_entry.tok_type {
          // these tokens are in First(declaration). Parse the declaration and a terminating semicolon
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => {
            let declaration = self.declaration(builder);
            if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error{..} = &declaration {
              declaration.print();
            }
            
            self.resync();
          },
          _ => break
        }
      } else {
        break;
      }
    }
    
    let begin_kw = self.parse_tok(tokens::begin_kw::BeginKW::start());
    if let ParserResult::Success(_) = begin_kw {
      // while next token is in First(statement), read in statement w/ semicolon terminator
      loop {
        if let Some(tok_entry) = self.lexer.peek() {
          match &tok_entry.tok_type {
            Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => {
              // if able to parse a statement, parse a terminating semicolon
              // program doesn't have a return type so neither program statements
              let statement = self.statement(builder, &Type::None);
              self.resync();
            },
            _ => break
          }
          
        } else {
          break;
        }
      }
      
      // parse the end kw
      let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
      if let ParserResult::Success(_) = end_kw {
        let program_kw = self.parse_tok(tokens::program_kw::ProgramKW::start());
        if let ParserResult::Success(tok_entry) = program_kw {
        
          // leave the current scope
          let popped_table = self.symbol_table_chain.pop();
          
          // debugging - print the values in this scope
          if let Some(table) = popped_table {
            Parser::print_symbol_table(String::from("Program scope"), &table);
          }
          
          return ParserResult::Success(tok_entry);
        } else { program_kw.print(); return program_kw; }
      } else { end_kw.print(); return end_kw; }
    } else { begin_kw.print(); return begin_kw; }
  }
  
  pub fn declaration(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
  
    let scope = if let ParserResult::Success(..) = self.parse_tok(tokens::global_kw::GlobalKW::start()) {
      Scope::Global
    } else {
      Scope::Local
    };
    
    if let Some(tok_entry) = self.lexer.peek() {
      match &tok_entry.tok_type {
        Token::ProcedureKW(_tok) => { return self.procedure_declaration(builder, &scope); },
        Token::VariableKW(_tok) => {return self.variable_declaration(builder, &scope); },
        Token::TypeKW(_tok) => { return self.type_declaration(builder, &scope); },
        _ => { return ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num, expected: String::from("(procedure|variable|type)"), actual: String::from(&tok_entry.chars[..])}; }
      }
      
    } else {
      return ParserResult::ErrUnexpectedEnd;
    }
  }
  
  pub fn procedure_declaration(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope) -> ParserResult {
  
    let mut builder = unsafe {
      core::LLVMCreateBuilder()
    };
  
    let procedure_header = self.procedure_header(&mut builder, scope);
    if let ParserResult::Success(return_type) = procedure_header {
      let procedure_body = self.procedure_body(&mut builder, scope, &return_type.r#type);
      if let ParserResult::Success(_) = procedure_body {
        return procedure_body;
      } else { procedure_body.print(); return procedure_body;}
    } else { procedure_header.print(); return procedure_header;}
  }
  
  pub fn procedure_header(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope) -> ParserResult {
    let procedure_kw = self.parse_tok(tokens::procedure_kw::ProcedureKW::start());
    if let ParserResult::Success(_) = procedure_kw {
    
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success(mut procedure_id) = identifier {
        let colon = self.parse_tok(tokens::colon::Colon::start());
        if let ParserResult::Success(_) = colon {
          let type_mark = self.type_mark();
          if let ParserResult::Success(mut result_type) = type_mark {
            let l_paren = self.parse_tok(tokens::parens::LParen::start());
            if let ParserResult::Success(_) = l_paren {
            
              let return_type = Parser::get_type(&result_type);
              result_type.r#type = return_type.clone();
              
              let mut procedure_type = Type::Procedure(vec![], Box::new(return_type));
              
              // create a new symbol table for the procedure scope
              self.symbol_table_chain.push(HashMap::new());
            
              // read optional parameter list
              let next_tok = self.lexer.peek();
              if let Some(tok_entry) = next_tok {
                if let Token::VariableKW(_) = &tok_entry.tok_type {
                  self.parameter_list(builder, scope, &mut procedure_type);
                }
              }
              
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success(_) = r_paren {
            
                // set the type of token to procedure
                procedure_id.r#type = procedure_type;
                
                // build the return type
                let mut ret_type;
                let mut params_type;
                
                if let Type::Procedure(params, ret) = &procedure_id.r#type {
                  ret_type = get_llvm_type(ret);
                  
                  let mut types = vec![];
                  
                  for param in params {
                    types.push(get_llvm_type(param));
                  }
                  
                  params_type = types.as_mut_ptr();
                  
                } else {
                  return ParserResult::ErrInvalidType{line_num: procedure_id.line_num,
                                                      expected: vec![Type::Procedure(vec![], Box::new(Type::None))],
                                                      actual: procedure_id.r#type};
                };
                
                
                // build the llvm function
                procedure_id.value_ref = unsafe {
                  // build the function type
                  let func_type = core::LLVMFunctionType(ret_type, params_type, 0, 0);
                
                  // add the function to the module
                  let func_name = c_str(&procedure_id.chars[..]);
                  let f = core::LLVMAddFunction(self.llvm_module, func_name, func_type);
                  
                  // set up basic block and builder for this function
                  let block = core::LLVMAppendBasicBlock(f, c_str("block"));
                  core::LLVMPositionBuilderAtEnd(*builder, block);
                  
                  f
                };
                
                // Note: Using Rc struct gives immutable multiple ownership
                // this means that the symbols in the table are immutable
                // may want to mutate to change the type, chars, tok_type of a symbol
                // but I believe this language does not require this since this information is given completely at declaration
                
                // if header is successful, save procedure_id to the symbol table
                let procedure_symbol = Rc::new(procedure_id);
                
                // include procedure_id in this new symbol table (allow recursive calls)
                let procedure_scope = Scope::Local;
                self.add_symbol(builder, &procedure_scope, Rc::clone(&procedure_symbol));
                
                // add procedure_id to the containing scope (pop procedure scope off then push back)
                if let Scope::Local = &scope {
                  if let Some(procedure_table) = self.symbol_table_chain.pop() {
                    self.add_symbol(builder, &scope, Rc::clone(&procedure_symbol));
                    self.symbol_table_chain.push(procedure_table);
                  }
                } else if let Scope::Global = &scope {
                  self.add_symbol(builder, &scope, Rc::clone(&procedure_symbol));
                }
              
                return ParserResult::Success(result_type);
                
              } else { r_paren.print(); return r_paren; }
            } else { l_paren.print(); return l_paren; }
          } else { type_mark.print(); return type_mark; }
        } else { colon.print(); return colon; }
      } else { identifier.print(); return identifier; }
    } else { procedure_kw.print(); return procedure_kw; }
  }
  
  pub fn type_mark(&mut self) -> ParserResult {
  
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = peek_tok {
      match tok_entry.tok_type {
        Token::IntegerKW(_) => { 
          if let Some(entry) = self.lexer.next() {
            return ParserResult::Success(entry); 
          } else {
            return ParserResult::ErrUnexpectedEnd;
          }
        },
        Token::FloatKW(_) => {
          if let Some(entry) = self.lexer.next() {
            return ParserResult::Success(entry);
          } else {
            return ParserResult::ErrUnexpectedEnd;
          }
        },
        Token::StringKW(_) => { 
          if let Some(entry) = self.lexer.next() {
            return ParserResult::Success(entry);
          } else {
            return ParserResult::ErrUnexpectedEnd;
          }
        },
        Token::BoolKW(_) => {
          if let Some(entry) = self.lexer.next() {
            return ParserResult::Success(entry);
          } else {
            return ParserResult::ErrUnexpectedEnd;
          }
        },
        Token::Identifier(_) => {
          if let Some(entry) = self.lexer.next() {
            return ParserResult::Success(entry);
          } else {
            return ParserResult::ErrUnexpectedEnd;
          }
        },
        Token::EnumKW(_) => {
          self.lexer.next();
          
          let l_brace = self.parse_tok(tokens::braces::LBrace::start());
          if let ParserResult::Success(_) = l_brace {
            let identifier = self.parse_tok(tokens::identifier::Identifier::start());
            if let ParserResult::Success(_) = identifier {
            
              loop {
                // optionally parse additional identifiers (delimited by comma)
                if let ParserResult::Success(_) = self.parse_tok(tokens::comma::Comma::start()) {
                  self.parse_tok(tokens::identifier::Identifier::start());
                } else {
                  break;
                }
              }
              
              let r_brace = self.parse_tok(tokens::braces::RBrace::start());
              if let ParserResult::Success(_) = r_brace {
                return r_brace;
              } else {
                r_brace.print();
                return r_brace;
              }
            } else {
              identifier.print();
              return identifier;
            }
          } else {
            l_brace.print();
            return l_brace;
          }
          
        },
        _ => { return ParserResult::ErrUnexpectedTok{line_num: tok_entry.line_num, expected: String::from("<some_type_kw>"), actual: String::from(&tok_entry.chars[..])}; }
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  pub fn parameter_list(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope, procedure: &mut Type) -> ParserResult {
    
    let parameter = self.parameter(builder, scope);
    if let ParserResult::Success(param_entry) = &parameter {
    
      // add this parameter type to the procedure type
      if let Type::Procedure(param_types, _) = procedure {
        param_types.push(Box::new(Parser::get_type(&param_entry)));
      } else {
        println!("Hmmm. non-Type::Procedure type passed into parameter_list");
      }
      
      // optionally parse another parameter list (delimited by comma)
      let comma = self.parse_tok(tokens::comma::Comma::start());
      if let ParserResult::Success(_) = comma {
        // call recursively to parse the rest of the list
        return self.parameter_list(builder, scope, procedure);
      } else {
        return parameter;
      }
      
    } else {
      parameter.print();
      return parameter;
    }
    
  }
  
  pub fn parameter(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope) -> ParserResult {
    return self.variable_declaration(builder, scope);
  }
  
  pub fn procedure_body(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope, return_type: &Type) -> ParserResult {
  
    // TODO break this out to its own function
    // parse an optional number of declarations delimited by semicolon
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        match &tok_entry.tok_type {
          // these tokens are in First(declaration). Parse the declaration and a terminating semicolon
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => {
            let declaration = self.declaration(builder);
            if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error{..} = declaration {
              declaration.print();
            }
            
            self.resync();
          },
          _ => break
        }
      } else {
        break;
      }
    }
    
    let begin_kw = self.parse_tok(tokens::begin_kw::BeginKW::start());
    if let ParserResult::Success(_) = begin_kw {
      // parse an optional number of statements
      // TODO unused, but use this variable to enforce that a return statement is found
      let does_return = false;
      loop {
        if let Some(tok_entry) = self.lexer.peek() {
          match &tok_entry.tok_type {
            Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => {
              let statement = self.statement(builder, return_type);
              self.resync();
            },
            _ => break
          }
          
        } else {
          break;
        }
      }
      
      let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
      if let ParserResult::Success(_) = end_kw {
        let procedure_kw = self.parse_tok(tokens::procedure_kw::ProcedureKW::start());
        if let ParserResult::Success(_) = procedure_kw {
        
          // leave the current scope
          let popped_scope = self.symbol_table_chain.pop();
          
          if let Some(table) = popped_scope {
            Parser::print_symbol_table(String::from("Procedure scope"), &table);
          }
        
          return procedure_kw;
          
        } else { return procedure_kw; }
      } else { return end_kw; }
    } else { return begin_kw; }
  }
  
  pub fn variable_declaration(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope) -> ParserResult {
    
    let variable_kw = self.parse_tok(tokens::variable_kw::VariableKW::start());
    if let ParserResult::Success(_) = variable_kw {
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success(mut variable_id) = identifier {
        let colon = self.parse_tok(tokens::colon::Colon::start());
        if let ParserResult::Success(_) = colon {
          let type_mark = self.type_mark();
          if let ParserResult::Success(mut variable_type) = type_mark {
            
            variable_type.r#type = Parser::get_type(&variable_type);
            
            variable_id.value_ref = unsafe {core::LLVMBuildAlloca(*builder, get_llvm_type(&variable_type.r#type), c_str(&variable_id.chars[..]))};
            
            // optionally parse a bound (making this variable an array)
            let l_bracket = self.parse_tok(tokens::brackets::LBracket::start());
            if let ParserResult::Success(_) = l_bracket {
              let bound = self.bound();
              if let ParserResult::Success(bound_entry) = bound {
                let r_bracket = self.parse_tok(tokens::brackets::RBracket::start());
                if let ParserResult::Success(_) = r_bracket {
                
                  // update the token type based on the type_mark
                  let arr_type = Parser::get_type(&variable_type);
                  let arr_size = match bound_entry.chars.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => 0
                  };
                  variable_id.r#type = Type::Array(Box::new(arr_type), arr_size);
                  
                  // if successful, add the variable to the symbol table
                  self.add_symbol(builder, scope, Rc::new(variable_id));
                
                  return ParserResult::Success(variable_type);
                } else {
                  return r_bracket;
                }
              } else {
                return bound;
              }
            }
            
            // update the token type baed on the type_mark
            variable_id.r#type = Parser::get_type(&variable_type);
            
            // if successful without bounds, also add to symbol table
            self.add_symbol(builder, scope, Rc::new(variable_id));
            
            return ParserResult::Success(variable_type);
            
          } else { return type_mark; }
        } else { return colon; }
      } else { return identifier; }
    } else { return variable_kw; }
  }
  
  pub fn bound(&mut self) -> ParserResult {
    
    // optionally parse a dash (minus)
    self.parse_tok(tokens::dash::Dash::start());
    
    return self.parse_tok(tokens::number::Number::start());
    
  }
  
  pub fn type_declaration(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope) -> ParserResult {
    let type_kw = self.parse_tok(tokens::type_kw::TypeKW::start());
    if let ParserResult::Success(_) = type_kw {
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success(mut type_id) = identifier {
        let is_kw = self.parse_tok(tokens::is_kw::IsKW::start());
        if let ParserResult::Success(_) = is_kw {
          let type_mark = self.type_mark();
          if let ParserResult::Success(type_entry) = &type_mark {
          
            // change the token entry type
            type_id.r#type = Type::Type(Box::new(Parser::get_type(&type_entry)));
            
            self.add_symbol(builder, scope, Rc::new(type_id));
          
            return type_mark;
          } else { return type_mark; }
        } else { return is_kw; }
      } else { return identifier; }
    } else { return type_kw; }
    
  }
  
  pub fn statement(&mut self, builder: &mut LLVMBuilderRef, return_type: &Type) -> ParserResult {
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = peek_tok {
      let result = match &tok_entry.tok_type {
        Token::Identifier(_) => self.assignment_statement(builder),
        Token::IfKW(_) => self.if_statement(builder, return_type),
        Token::ForKW(_) => self.loop_statement(builder, return_type),
        Token::ReturnKW(_) => self.return_statement(builder, return_type),
        _ => { return ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num, expected: String::from("(<identifier>|if|for|return)"), actual: String::from(&tok_entry.chars[..])} }
      };
      
      if let ParserResult::Success(_) = result {
        return result;
      } else {
        result.print();
        return result;
      }
      
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  pub fn procedure_call_w_identifier(&mut self, builder: &mut LLVMBuilderRef, identifier: ParserResult, resolve_type: &Type) -> ParserResult {
    if let ParserResult::Success(mut procedure_id) = identifier {
    
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success(_) = l_paren {
        
        // look up the procedure (token_entry)
        let procedure = match self.get_symbol(&procedure_id.chars) {
          Some(val) => val,
          None => return ParserResult::ErrSymbolNotFound{name: String::from(&procedure_id.chars[..]), line_num: procedure_id.line_num}
        };
        
        // check that the retrieved symbol is a procedure
        let (procedure_params, procedure_ret) = match &procedure.r#type {
          Type::Procedure(params, ret) => (params.clone(), *ret.clone()),
          _ => {
            return ParserResult::ErrInvalidType{line_num: procedure_id.line_num,
                                                    expected: vec![Type::Procedure(vec![], Box::new(Type::None))],
                                                    actual: procedure.r#type.clone()};
            
          }
        };
        
        let llvm_procedure = procedure.value_ref.clone();
        
        // parse optional argument list (includes checking types)
        let mut arg_list = vec![];
        self.argument_list(builder, &mut arg_list, procedure_params.iter());
        
        let r_paren = self.parse_tok(tokens::parens::RParen::start());
        if let ParserResult::Success(entry) = &r_paren {
          procedure_id.r#type = procedure_ret;
          
          // llvm function call
          let res = unsafe {
            let result = core::LLVMBuildCall(*builder, llvm_procedure, arg_list.as_mut_ptr(), arg_list.len() as u32, null_str());
            
            result
          };
          
          procedure_id.value_ref = res;
          
          return ParserResult::Success(procedure_id);
        } else { return r_paren; }
      } else { return l_paren; }
    } else { return identifier; }
  }
  
  // this is currently unused since procedure calls currently only occur ambiguously with names (in the factor parse rule)
  pub fn procedure_call(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
  
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    return self.procedure_call_w_identifier(builder, identifier, resolve_type);
    
  }
  
  pub fn name_w_identifier(&mut self, builder: &mut LLVMBuilderRef, mut identifier: ParserResult) -> ParserResult {
    if let ParserResult::Success(id_entry) = &mut identifier {
    
      // make sure the identifier exists
      let symbol = if let Some(value) = self.get_symbol(&id_entry.chars) {
        value
      } else {
        return ParserResult::ErrSymbolNotFound{line_num: id_entry.line_num, name: String::from(&id_entry.chars[..])};
      };
      
      // extract the type of the identifier
      let val_type = symbol.r#type.clone();
      
      id_entry.r#type = val_type;
    
      // prepare a value to load into
      let value = unsafe { core::LLVMBuildLoad(*builder, symbol.value_ref, c_str(&id_entry.chars[..])) };
      id_entry.value_ref = value;
    
      // optionally parse square bracket
      let peek_tok = self.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        // if next up is an LBracket, commit to parsing this optional portion
        if let Token::LBracket(_) = &tok_entry.tok_type {
          // consume the l_bracket
          self.lexer.next();
          
          let arr_idx_type = Type::Integer;
          let expression = self.expression(builder, &arr_idx_type);
          if let ParserResult::Success(_) = expression {
            let r_bracket = self.parse_tok(tokens::brackets::RBracket::start());
            if let ParserResult::Success(_) = r_bracket {
            
              // if indexing, check that this is actually an array
              if let Type::Array(..) = id_entry.r#type {} else {
                return ParserResult::ErrInvalidType{line_num: id_entry.line_num,
                                                    expected: vec![Type::Array(Box::new(Type::None), 0)],
                                                    actual: id_entry.r#type.clone()};
              }
              
              return identifier;
            } else { return r_bracket; }
          } else { return expression; }
        }
      }
      
      return identifier;
      
    } else { return identifier; }
  }
  
  pub fn name(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    return self.name_w_identifier(builder, identifier);
  }
  
  pub fn term(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
  
    // define function for factored parse rule
    fn _term<'a>(slf: &mut Parser<'a>, builder: &mut LLVMBuilderRef, resolve_type: &Type, mut left: TokenEntry) -> ParserResult {
      // accept either a '*' or '/'
      let asterisk = slf.parse_tok(tokens::asterisk::Asterisk::start());
      if let ParserResult::Success(_) = asterisk {
        let factor = slf.factor(builder, resolve_type);
        if let ParserResult::Success(factor_entry) = factor {
        
          // ensure that left and factor are integer or float (both compatible with float)
          let float_type = Type::Float;
          if !Parser::is_compatible(&float_type, &left.r#type) {
            return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                expected: vec![float_type, Type::Integer],
                                                actual: left.r#type};
          } else if !Parser::is_compatible(&float_type, &factor_entry.r#type) {
            return ParserResult::ErrInvalidType{line_num: factor_entry.line_num,
                                                expected: vec![float_type, Type::Integer],
                                                actual: factor_entry.r#type};
          } else {
            // fold factor_entry into left
            
            // upgrade the accumulated type to a float
            left.r#type = float_type;
            left.line_num = factor_entry.line_num;
          }
        
          return _term(slf, builder, resolve_type, left);
        } else { return factor; }
      }
      
      let slash = slf.parse_tok(tokens::slash::Slash::start());
      if let ParserResult::Success(_) = slash {
        let factor = slf.factor(builder, resolve_type);
        if let ParserResult::Success(factor_entry) = factor {
        
          // ensure that left and factor_entry types are compatible here
          let float_type = Type::Float;
          if !Parser::is_compatible(&float_type, &factor_entry.r#type) {
            return ParserResult::ErrInvalidType{line_num: factor_entry.line_num,
                                                expected: vec![float_type, Type::Integer],
                                                actual: factor_entry.r#type};
          } else if !Parser::is_compatible(&float_type, &left.r#type) {
            return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                expected: vec![float_type, Type::Integer],
                                                actual: left.r#type};
          } else {
            // fold the factor_entry value into left
            
            // upgrade the type to a float
            left.r#type = float_type;
            left.line_num = factor_entry.line_num;
          }
        
          return _term(slf, builder, resolve_type, left);
        } else { return factor; }
      }
      
      // allow _term to parse nothing
      // in this case, just return the unmodified lhs
      return ParserResult::Success(left);
      
    }
  
    // read bottomed-out factor rule
    let factor = self.factor(builder, resolve_type);
    if let ParserResult::Success(left) = factor {
      return _term(self, builder, resolve_type, left);
    } else { return factor; }
    
  }
  
  pub fn relation(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
    fn _relation<'a>(slf: &mut Parser<'a>, builder: &mut LLVMBuilderRef, resolve_type: &Type, mut left: TokenEntry) -> ParserResult {
    
      let int_type = Type::Integer;
      let string_type = Type::String;
    
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::LT(_) => {
            let lt = slf.parse_tok(tokens::lt::LT::start());
            if let ParserResult::Success(lt_entry) = lt {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
              
                if !Parser::is_compatible(&int_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool],
                                                      actual: left.r#type};
                }
                
                // make sure that left is compatible with term_entry
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // fold the term value into left
                // this will turn left into a bool type
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return lt; }
          },
          Token::GTE(_) => {
            let gte = slf.parse_tok(tokens::gte::GTE::start());
            if let ParserResult::Success(gte_entry) = gte {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
              
                // check that left and term_entry have compatible types
                if !Parser::is_compatible(&int_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // if compatible, fold term_entry into left
                // which makes it a bool
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return gte; }
          },
          Token::LTE(_) => {
            let lte = slf.parse_tok(tokens::lte::LTE::start());
            if let ParserResult::Success(lte_entry) = lte {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
              
                // check that left and term_entry have compatible types
                if !Parser::is_compatible(&int_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // fold term_entry into left
                // which converts the type of left to bool
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return lte; }
          },
          Token::GT(_) => {
            let gt = slf.parse_tok(tokens::gt::GT::start());
            if let ParserResult::Success(gt_entry) = gt {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
                // check that term_entry type is compatible with left (so that a comparison is possible)
                if !Parser::is_compatible(&int_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // fold term entry into left
                // which converts the type to bool
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return gt; }
          },
          Token::EQ(_) => {
            let eq = slf.parse_tok(tokens::eq::EQ::start());
            if let ParserResult::Success(eq_entry) = eq {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
              
                if !(Parser::is_compatible(&int_type, &left.r#type) || Parser::is_compatible(&string_type, &left.r#type)) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool, string_type],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // fold term_entry into left by equality op
                // converts left into a bool
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return eq; }
          },
          Token::NEQ(_) => {
            let neq = slf.parse_tok(tokens::neq::NEQ::start());
            if let ParserResult::Success(neq_entry) = neq {
              let term = slf.term(builder, resolve_type);
              if let ParserResult::Success(term_entry) = term {
              
                if !(Parser::is_compatible(&int_type, &left.r#type) || Parser::is_compatible(&string_type, &left.r#type)) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![int_type, Type::Float, Type::Bool, string_type],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&left.r#type, &term_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: term_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: term_entry.r#type};
                }
                
                // fold term_entry into left via neq op
                // converts left to a bool
                left.r#type = Type::Bool;
                left.line_num = term_entry.line_num;
              
                return _relation(slf, builder, resolve_type, left);
              } else { return term; }
            } else { return neq; }
          }
          _ => { 
            // allow nothing to be parsed (allow lambda production)
            // this is the base case for this recursive function
            // will keep recursing until next token is not a comparison (<, >, etc.)
            return ParserResult::Success(left); 
          }
        }
      } else {
        // similar to catch-all, allow nothing to be parsed
        return ParserResult::Success(left);
      }
    }
    
    let term = self.term(builder, resolve_type);
    if let ParserResult::Success(term_entry) = term {
      let rel = _relation(self, builder, resolve_type, term_entry);
      return rel;
    } else { return term; }
  }
  
  pub fn arith_op(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
    fn _arith_op(slf: &mut Parser, builder: &mut LLVMBuilderRef, resolve_type: &Type, mut left: TokenEntry) -> ParserResult {
    
      let float_type = Type::Float;
    
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::Plus(_) => {
            let plus = slf.parse_tok(tokens::plus::Plus::start());
            if let ParserResult::Success(_) = plus {
              let relation = slf.relation(builder, resolve_type);
              if let ParserResult::Success(relation_entry) = relation {
              
                if !Parser::is_compatible(&float_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![float_type, Type::Integer],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&float_type, &relation_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: relation_entry.line_num,
                                                      expected: vec![float_type, Type::Integer],
                                                      actual: relation_entry.r#type};
                }
                
                // fold relation entry into left
                
                // convert the type if relation_entry is a float
                if let Type::Float = &relation_entry.r#type {
                  left.r#type = float_type;
                }
                
                left.line_num = relation_entry.line_num;
              
                return _arith_op(slf, builder, resolve_type, left);
              } else {
                return relation;
              }
            } else {
              return plus;
            }
          },
          Token::Dash(_) => {
            let dash = slf.parse_tok(tokens::dash::Dash::start());
            if let ParserResult::Success(_) = dash {
              let relation = slf.relation(builder, resolve_type);
              if let ParserResult::Success(relation_entry) = relation {
              
                // check left and relation_entry have the correct types
                if !Parser::is_compatible(&float_type, &left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: left.line_num,
                                                      expected: vec![float_type, Type::Integer],
                                                      actual: left.r#type};
                }
                
                if !Parser::is_compatible(&float_type, &relation_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: relation_entry.line_num,
                                                      expected: vec![float_type, Type::Integer],
                                                      actual: relation_entry.r#type};
                }
                
                // fold relation entry into left by '-'
                // type will be float if either arg is a float. otherwise int
                if let Type::Float = &relation_entry.r#type {
                  left.r#type = float_type;
                }
                left.line_num = relation_entry.line_num;
              
                return _arith_op(slf, builder, resolve_type, left);
              } else {
                return relation;
              }
            } else {
              return dash;
            }
          },
          _ => {
            // base case: allow nothing to be parsed if '+' and '-' not found
            return ParserResult::Success(left);
          }
        }
      } else {
        // base case
        return ParserResult::Success(left);
      }
    }
    
    // parse the initial relation where the recursion bottoms out
    let relation = self.relation(builder, resolve_type);
    if let ParserResult::Success(relation_entry) = relation {
      return _arith_op(self, builder, &Type::Float, relation_entry);
    } else {
      return relation;
    }
  }
  
  pub fn expression(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
    fn _expression(slf: &mut Parser, builder: &mut LLVMBuilderRef, resolve_type: &Type, mut left: TokenEntry) -> ParserResult {
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::Ampersand(_) => {
            let ampersand = slf.parse_tok(tokens::ampersand::Ampersand::start());
            if let ParserResult::Success(_) = ampersand {
              let arith_op = slf.arith_op(builder, resolve_type);
              if let ParserResult::Success(arith_op_entry) = arith_op {
              
                // check that arith_op_entyr and left are the same type
                if mem::discriminant(&arith_op_entry.r#type) != mem::discriminant(&left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: arith_op_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: arith_op_entry.r#type};
                }
                
                // check that type is either int or bool (for bitwise or logical op)
                if let Type::Bool = &arith_op_entry.r#type {
                  // perform logical operation (returns bool)
                  
                } else if let Type::Integer = &arith_op_entry.r#type {
                  // perform bitwise operation (return integer)
                  
                } else {
                  // invalid type
                  return ParserResult::ErrInvalidType{line_num: arith_op_entry.line_num,
                                                      expected: vec![Type::Bool, Type::Integer],
                                                      actual: arith_op_entry.r#type};
                }
                
                left.line_num = arith_op_entry.line_num;
              
                return _expression(slf, builder, resolve_type, left);
              } else {
                return arith_op;
              }
            } else {
              return ampersand;
            }
          },
          Token::Pipe(_) => {
            let pipe = slf.parse_tok(tokens::pipe::Pipe::start());
            if let ParserResult::Success(_) = pipe {
              let arith_op = slf.arith_op(builder, resolve_type);
              if let ParserResult::Success(arith_op_entry) = arith_op {
              
                // check that arith_op_entyr and left are the same type
                if mem::discriminant(&arith_op_entry.r#type) != mem::discriminant(&left.r#type) {
                  return ParserResult::ErrInvalidType{line_num: arith_op_entry.line_num,
                                                      expected: vec![left.r#type],
                                                      actual: arith_op_entry.r#type};
                }
                
                // check that type is either int or bool (for bitwise or logical op)
                if let Type::Bool = &arith_op_entry.r#type {
                  // perform logical operation (returns bool)
                  
                } else if let Type::Integer = &arith_op_entry.r#type {
                  // perform bitwise operation (return integer)
                  
                } else {
                  // invalid type
                  return ParserResult::ErrInvalidType{line_num: arith_op_entry.line_num,
                                                      expected: vec![Type::Bool, Type::Integer],
                                                      actual: arith_op_entry.r#type};
                }
                
                left.line_num = arith_op_entry.line_num;

                return _expression(slf, builder, resolve_type, left);
              } else {
                return arith_op;
              }
            } else {
              return pipe;
            }
          },
          _ => {
            // base case: if non-matching token is hit, do not parse (lambda-production)
            return ParserResult::Success(left);
          }
        }
      } else {
        // base case: allow nothing to be parsed
        return ParserResult::Success(left);
      }
    }
    
    // optionally parse a 'not' kw
    let negate = match self.parse_tok(tokens::not_kw::NotKW::start()) {
      ParserResult::Success(_) => true,
      _ => false
    };
    
    let arith_op = self.arith_op(builder, resolve_type);
    if let ParserResult::Success(arith_op_entry) = arith_op {
      return _expression(self, builder, resolve_type, arith_op_entry);
    } else { return arith_op; }
  }
  
  pub fn argument_list(&mut self, builder: &mut LLVMBuilderRef, arg_list: &mut Vec<LLVMValueRef>, mut param_types: Iter<Box<Type>>) -> ParserResult {
  
    // compare arguments to procedure parameters
    let curr_arg_type;
    if let Some(param_type) = param_types.next() {
      curr_arg_type = param_type;
    } else {
      return ParserResult::Error{line_num: 0, msg: String::from("Expected more arguments")};
    }
  
    let expression = self.expression(builder, curr_arg_type);
    if let ParserResult::Success(entry) = &expression {
    
      // cast value if needed
    
      arg_list.push(entry.value_ref);
    
      // optionally parse the rest
      let comma = self.parse_tok(tokens::comma::Comma::start());
      if let ParserResult::Success(..) = comma {
        return self.argument_list(builder, arg_list, param_types);
      }
      
      return expression;
    } else { return expression; }
  }
  
  pub fn assignment_statement(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
    let destination = self.destination(builder);
    if let ParserResult::Success(mut dest_id) = destination {
    
      // look up the destination in the symbol table to retrieve the type
      if let Some(dest) = self.get_symbol(&dest_id.chars) {
        let dest_type = dest.r#type.clone();
        let dest_value_ref = dest.value_ref.clone();
        
        let assign = self.parse_tok(tokens::assign::Assign::start());
        if let ParserResult::Success(assign_entry) = assign {
          let expression = self.expression(builder, &dest_type);
          if let ParserResult::Success(mut expr_entry) = expression {
          
            // enforce that the expression type is compatible with destination type
            if !Parser::is_compatible(&dest_type, &expr_entry.r#type) {
              return ParserResult::ErrInvalidType{line_num: assign_entry.line_num,
                                                  expected: vec![dest_type],
                                                  actual: expr_entry.r#type.clone()};
            }
            
            // coerce the type if necessary
            // cast the expression to the return type
            let coercion = Parser::coerce(builder, &expr_entry.r#type, &dest_type, &mut expr_entry.value_ref);
            if let Result::Ok(coerced_val) = coercion {
              expr_entry.value_ref = coerced_val;
            }
            
            // store the value in alloca
            unsafe {
              // TODO check that the value is a pointer (alloca)
            
              // store the expression value
              core::LLVMBuildStore(*builder, expr_entry.value_ref, dest_value_ref);
            };
          
            return ParserResult::Success(dest_id);
          } else { return expression; }
        } else { return assign; }
        
      } else { return ParserResult::ErrSymbolNotFound{name: dest_id.chars, line_num: dest_id.line_num}; }
      
    } else { return destination; }
  }
  
  pub fn destination(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    if let ParserResult::Success(_) = identifier {
      // optionally parse an index to this value
      let l_bracket = self.parse_tok(tokens::brackets::LBracket::start());
      if let ParserResult::Success(_) = l_bracket {
        let idx_type = Type::Integer;
        let expression = self.expression(builder, &idx_type);
        if let ParserResult::Success(_) = expression {
          if let ParserResult::Success(_) = self.parse_tok(tokens::brackets::RBracket::start()) {
            return identifier;
          }
        } else {
          return expression;
        }
      }
      
      return identifier;
      
    } else { return identifier; }
  }
  
  pub fn if_statement(&mut self, builder: &mut LLVMBuilderRef, return_type: &Type) -> ParserResult {
    let if_kw = self.parse_tok(tokens::if_kw::IfKW::start());
    if let ParserResult::Success(_) = if_kw {
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success(_) = l_paren {
      
        // if statement expressions should evaluate to a boolean
        let if_expr_type = Type::Bool;
        let expression = self.expression(builder, &if_expr_type);
        if let ParserResult::Success(expr_entry) = expression {
          let r_paren = self.parse_tok(tokens::parens::RParen::start());
          if let ParserResult::Success(r_paren_entry) = r_paren {
          
            // check that that the expression type is compatible with bool
            if !Parser::is_compatible(&if_expr_type, &expr_entry.r#type) {
              return ParserResult::ErrInvalidType{line_num: r_paren_entry.line_num,
                                                  expected: vec![if_expr_type],
                                                  actual: expr_entry.r#type};
            }
          
            let then_kw = self.parse_tok(tokens::then_kw::ThenKW::start());
            if let ParserResult::Success(_) = then_kw {
              
              // parse an arbitrary number of statements delimited by ';'
              loop {
                if let Some(tok_entry) = self.lexer.peek() {
                  match tok_entry.tok_type {
                    Token::ElseKW(_) | Token::EndKW(_) => break,
                    _ => {
                      let statement = self.statement(builder, return_type);
                      self.resync();
                    }
                  }
                } else {
                  break;
                }
              }
              
              // optionally parse else statement
              let else_kw = self.parse_tok(tokens::else_kw::ElseKW::start());
              if let ParserResult::Success(_) = else_kw {
                // parse an arbitrary number of statements delimited by ';'
                loop {
                  if let Some(tok_entry) = self.lexer.peek() {
                    match tok_entry.tok_type {
                      Token::EndKW(_) => break,
                      _ => {
                        let statement = self.statement(builder, return_type);
                        self.resync();
                      }
                    }
                  } else {
                    break;
                  }
                }
              }
              
              let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
              if let ParserResult::Success(_) = end_kw {
                return self.parse_tok(tokens::if_kw::IfKW::start());
              } else { 
                return end_kw;
              }
            } else { return then_kw; }
          } else { return r_paren; }
        } else { return expression; }
      } else { return l_paren; }
    } else { return if_kw; }
  }
  
  pub fn loop_statement(&mut self, builder: &mut LLVMBuilderRef, return_type: &Type) -> ParserResult {
    let for_kw = self.parse_tok(tokens::for_kw::ForKW::start());
    if let ParserResult::Success(_) = for_kw {
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success(_) = l_paren {
        let assignment_statement = self.assignment_statement(builder);
        if let ParserResult::Success(_) = assignment_statement {
          let semicolon = self.parse_tok(tokens::semicolon::Semicolon::start());
          if let ParserResult::Success(_) = semicolon {
            
            // loop statement expressions should evaluate to a boolean (invariant)
            let loop_expr_type = Type::Bool;
            let expression = self.expression(builder, &loop_expr_type);
            if let ParserResult::Success(expr_entry) = expression {
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success(r_paren_entry) = r_paren {
              
                // check that that the expression type is compatible with bool
                if !Parser::is_compatible(&loop_expr_type, &expr_entry.r#type) {
                  return ParserResult::ErrInvalidType{line_num: r_paren_entry.line_num,
                                                      expected: vec![loop_expr_type],
                                                      actual: expr_entry.r#type};
                }
                
                // parse an arbitrary number of statements delimited by ';'
                loop {
                  if let Some(tok_entry) = self.lexer.peek() {
                    match tok_entry.tok_type {
                      Token::EndKW(_) => break,
                      _ => {
                        let statement = self.statement(builder, return_type);
                        self.resync();
                      }
                    }
                  } else {
                    break;
                  }
                }
                
                let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
                if let ParserResult::Success(_) = end_kw {
                  return self.parse_tok(tokens::for_kw::ForKW::start());
                } else { return end_kw; }
              } else { return r_paren; }
            } else { return expression; }
          } else { return semicolon; }
        } else { return assignment_statement; }
      } else { return l_paren; }
    } else { return for_kw; }
  }
  
  pub fn return_statement(&mut self, builder: &mut LLVMBuilderRef, return_type: &Type) -> ParserResult {
    let return_kw = self.parse_tok(tokens::return_kw::ReturnKW::start());
    if let ParserResult::Success(return_entry) = return_kw {
      let expression = self.expression(builder, return_type);
      if let ParserResult::Success(mut expr_entry) = expression {
        // check that that the expression type is compatible with expected type
        if !Parser::is_compatible(&return_type, &expr_entry.r#type) {
          return ParserResult::ErrInvalidType{line_num: return_entry.line_num,
                                              expected: vec![return_type.clone()],
                                              actual: expr_entry.r#type.clone()};
        }
        
        // cast the expression to the return type
        let coercion = Parser::coerce(builder, &expr_entry.r#type, return_type, &mut expr_entry.value_ref);
        if let Result::Ok(coerced_val) = coercion {
          expr_entry.value_ref = coerced_val;
        }
        
        let ret = unsafe {
          core::LLVMBuildRet(*builder, expr_entry.value_ref)
        };
        
        return ParserResult::Success(expr_entry);
        
      } else {
        return expression;
      }
    } else { return return_kw; }
  }

  pub fn procedure_call_or_name(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
    // this could be a procedure call or a name based on the next token
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    
    let peek_tok = self.lexer.peek();
    
    if let Some(tok_entry) = &peek_tok {
      if let Token::LParen(_) = &tok_entry.tok_type {
        return self.procedure_call_w_identifier(builder, identifier, resolve_type);
      }
    }
    
    return self.name_w_identifier(builder, identifier);
    
  }
  
  pub fn name_or_number(&mut self, builder: &mut LLVMBuilderRef) -> ParserResult {
    
    // optionally parse a dash
    // TODO act on this value. May need to negate the number and reject non-multipliable
    let negate = if let ParserResult::Success(_) = self.parse_tok(tokens::dash::Dash::start()) {
      true
    } else {
      false
    };
    
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = &peek_tok {
      let value = match &tok_entry.tok_type {
        Token::Identifier(_) => { self.name(builder) },
        Token::Number(_) => { self.parse_tok(tokens::number::Number::start()) },
        _ => { return ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num, expected: String::from("(<identifier>|<number>)"), actual: String::from(&tok_entry.chars[..])}; }
      };
      
      if let ParserResult::Success(value_entry) = &value {
        // TODO apply negation here
      
        return value;
      } else { return value; }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  pub fn factor(&mut self, builder: &mut LLVMBuilderRef, resolve_type: &Type) -> ParserResult {
    // peek at next token to decide what type of factor this will be
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = &peek_tok {
      match &tok_entry.tok_type {
        Token::LParen(_) => {
          // resolve to a subexpression
          let l_paren = self.parse_tok(tokens::parens::LParen::start());
          if let ParserResult::Success(_) = l_paren {
            let expression = self.expression(builder, resolve_type);
            if let ParserResult::Success(_) = expression {
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success(_) = r_paren {
                return expression;
              } else { return r_paren; }
            } else { return expression; }
          } else { return l_paren; }
        },
        Token::Identifier(_) => { 
          return self.procedure_call_or_name(builder, resolve_type);
        },
        Token::Dash(_) => {
          // this could be a name or number depending on the next token
          return self.name_or_number(builder);
        },
        Token::String(_) => {
          return self.parse_tok(tokens::string::String::start());
        },
        Token::Number(_) => {
          return self.parse_tok(tokens::number::Number::start()); 
        }
        Token::TrueKW(_) => {
          return self.parse_tok(tokens::true_kw::TrueKW::start());
        },
        Token::FalseKW(_) => { return self.parse_tok(tokens::false_kw::FalseKW::start()); },
        _ => {
          return ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num,
                                                  expected: String::from("('('|<identifier>|'-'|<number>|<string>|true|false)"),
                                                  actual: String::from(&tok_entry.chars[..])};

        }
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  // TODO get this to use generics so that a dummy target token doesn't need to be passed in
  pub fn parse_tok(&mut self, target: Token) -> ParserResult {
    if let Some(tok_entry) = self.lexer.peek() {
      // if the next token matches the target, consume and return success result
      if mem::discriminant(&tok_entry.tok_type) == mem::discriminant(&target) {
        if let Some(tok_entry) = self.lexer.next() {
          return ParserResult::Success(tok_entry);
        } else {
          return ParserResult::ErrUnexpectedEnd;
        }
      } else { return ParserResult::ErrUnexpectedTok {line_num: tok_entry.line_num,
                                                      expected: String::from(target.get_example()),
                                                      actual: String::from(&tok_entry.chars[..])};
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  // consume tokens until a semicolon is hit, and then consume the semicolon
  pub fn resync(&mut self) -> ParserResult {
    let mut missed_semicolon = false;
    loop {
      let result = self.parse_tok(tokens::semicolon::Semicolon::start());
      
      match result {
        ParserResult::Success(_) => return result,
        ParserResult::ErrUnexpectedEnd => {
          result.print();
          return result;
        },
        _ => {
          if !missed_semicolon {
            result.print();
            missed_semicolon = true;
          }
          self.lexer.next();
        }
      }
    }
  }
  
  // return type based on the type mark token
  pub fn get_type(variable_entry: &TokenEntry) -> Type {
    return match variable_entry.tok_type {
      Token::EnumKW(_) => Type::Enum,
      Token::IntegerKW(_) => Type::Integer,
      Token::FloatKW(_) => Type::Float,
      Token::StringKW(_) => Type::String,
      Token::BoolKW(_) => Type::Bool,
      _ => Type::None
    };
  }
  
  pub fn get_symbol(&self, name: &String) -> Option<&TokenEntry> {
    // check the local table
    if let Some(local_table) = self.symbol_table_chain.last() {
      if let Some(symbol) = local_table.get(name) {
        return Some(symbol);
      } else if let Some(global_table) = self.symbol_table_chain.first() {
        if let Some(symbol) = global_table.get(name) {
          return Some(symbol);
        } else {
          return None;
        }
      } else {
        println!("Hmm. Symbol table not found");
        return None;
      }
    } else {
      println!("Hmm. Symbol table not found");
      return None;
    }
  }
  
  pub fn add_symbol(&mut self, builder: &mut LLVMBuilderRef, scope: &Scope, tok_entry: Rc<TokenEntry>) {
    match scope {
      Scope::Local => {
        if let Some(table) = self.symbol_table_chain.last_mut() {
          table.insert(String::from(&tok_entry.chars[..]), tok_entry);
        }
      },
      Scope::Global => {
        if let Some(table) = self.symbol_table_chain.first_mut() {
          table.insert(String::from(&tok_entry.chars[..]), tok_entry);
        }
      }
    }
  }
  
  pub fn add_builtins(&mut self, builder: &mut LLVMBuilderRef) {
    let (get_bool, put_bool) = builtins::bool::initialize_bool_funcs(self.llvm_module);
    self.add_builtin(builder, get_bool);
    self.add_builtin(builder, put_bool);
    
    let (get_integer, put_integer) = builtins::integer::initialize_integer_funcs(self.llvm_module);
    self.add_builtin(builder, get_integer);
    self.add_builtin(builder, put_integer);
    
    let (get_float, put_float) = builtins::float::initialize_float_funcs(self.llvm_module);
    self.add_builtin(builder, get_float);
    self.add_builtin(builder, put_float);
  }
  
  pub fn add_builtin(&mut self, builder: &mut LLVMBuilderRef, builtin: TokenEntry) {
    self.add_symbol(builder, &Scope::Global, Rc::new(builtin));
  }
  
  pub fn coerce(builder: &mut LLVMBuilderRef, from_type: &Type, to_type: &Type, value: &mut LLVMValueRef) -> Result<LLVMValueRef, ()> {
  
    if mem::discriminant(from_type) == mem::discriminant(to_type) {
      return Ok(*value);
    }
  
    return match (from_type, to_type) {
      (Type::Integer, Type::Bool) => {
        let res = unsafe {
          let is_true = core::LLVMBuildICmp(*builder, llvm_sys::LLVMIntPredicate::LLVMIntNE, *value, LLVMConstInt(core::LLVMInt32Type(), 0, 0), null_str());
          core::LLVMBuildZExt(*builder, is_true, core::LLVMInt32Type(), null_str())
        };
        
        Ok(res)
      },
      (Type::Integer, Type::Float) => Ok(unsafe { core::LLVMBuildSIToFP(*builder, *value, get_llvm_type(to_type), null_str()) }),
      (Type::Float, Type::Integer) => Ok(unsafe { core::LLVMBuildFPToSI(*builder, *value, get_llvm_type(to_type), null_str()) }),
      (Type::Bool, Type::Integer) => Ok(unsafe { core::LLVMBuildIntCast(*builder, *value, get_llvm_type(to_type), null_str()) }),
      _ => {
        println!("error while coercing");
        Err(())
      }
    };
  }
  
  fn is_compatible(expected_type: &Type, actual_type: &Type) -> bool {
    return match (expected_type, actual_type) {
      (Type::Integer, Type::Bool) | (Type::Bool, Type::Integer) => true,
      (Type::Integer, Type::Float) | (Type::Float, Type:: Integer) => true,
      (Type::Array(el_type_a, size_a), Type::Array(el_type_b, size_b)) => {
        Parser::is_compatible(el_type_a, el_type_b) && size_a == size_b
      },
      (Type::Array(el_type, size), other_type) => {
        Parser::is_compatible(el_type, other_type)
      },
      (other_type, Type::Array(el_type, el_size)) => {
        Parser::is_compatible(el_type, other_type)
      }
      (a, b) => {
        mem::discriminant(a) == mem::discriminant(b)
      }
    }
  }
  
  fn print_symbol_table(name: String, table: &HashMap<String, Rc<TokenEntry>>) {
    // debugging - print contents of the table
    println!("Printing variables in table for scope: {}", name);
    for key in table.keys() {
      if let Some(value) = table.get(key) {
        let type_str = value.r#type.to_string();
        
        println!("key: {} ({})", key, type_str);
      }
    }
    
    println!("\n");
    
  }
  
}

// TODO add chaining behavior so that we can chain parse rules until a failure is reached. This would really simplify this code by flattening all these nested ifs
pub enum ParserResult {
  ErrUnexpectedEnd,
  ErrUnexpectedTok{ expected: String, actual: String, line_num: u32},
  ErrSymbolNotFound{name: String, line_num: u32},
  ErrInvalidType{line_num: u32, expected: Vec<Type>, actual: Type},
  Error{line_num: u32, msg: String},
  Success(TokenEntry),
}

impl ParserResult {
  pub fn print(&self) {
    match self {
      ParserResult::ErrUnexpectedEnd => println!("Unexpected end of program."),
      ParserResult::ErrUnexpectedTok{line_num, expected, actual} => println!("({}) - Unexpected token - Expected: '{}', got: '{}'", line_num, expected, actual),
      ParserResult::ErrSymbolNotFound{line_num, name} => println!("({}) - Symbol undefined: '{}'", line_num, name),
      ParserResult::ErrInvalidType{line_num, expected, actual} => {
        let mut expected_str = String::new();
        for r#type in expected {
          expected_str.push_str(&r#type.to_string()[..]);
          expected_str.push_str(", ");
        }
        println!("({}) - Unexpected type: '{}', expected: [{}]", line_num, actual.to_string(), expected_str);
      },
      ParserResult::Error{line_num, msg} => println!("({}) - Error: {}", line_num, msg),
      ParserResult::Success(entry) => println!("({}) - Success", entry.line_num)
    }
  }
}

pub enum Scope {
  Local,
  Global
}