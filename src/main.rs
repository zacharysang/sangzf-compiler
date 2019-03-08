use std::env;
use std::fs::File;
use std::io::*;

// expose token utilities (State struct, Token trait)
mod tokenize;

mod tokens;

mod lexer;
mod parser;

fn main() {

  // mutable since we will want to remove the program_name arg
  let mut args : Vec<String> = env::args().collect();
  
  let mut program_name = String::from("test_programs/correct/source.src");
  if args.len() == 2 {
    program_name = args.remove(1);
  }
  
  // test program
  let mut program_file = File::open(program_name).expect("Could not open file");
  let mut program = String::new();
  
  program_file.read_to_string(&mut program).expect("Could not read file");
  
  // should have 51 tokens
  //let program = String::from(" program procedure global   ??? variable begin end is type integer float string bool enum if then else for return not true false . ; ( ) , { } - & + < > <= >= == != * / [ ] | := abcdef 1234 898.99 \"stringgoeshere\" \"fancie$t  string_g0es\n\nhere\t\"");
  
  // should have 3 tokens
  //let program = String::from("abc/**this is /*///*a***/*/ doc*/+bcd");
  
  //program = String::from("program p Is procedure proc : enum{a,b,c}(variable a : integer, variable b: bool, variable c: string, variable d: float) begin end procedure;  begin end program.");
  
  let program_chars = program.chars().peekable();
  
  let mut parser = parser::Parser::new(program_chars);
  
  parser.program();
  
}