use std::env;
use std::fs::File;
use std::io::*;

// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// expose lexer
mod lexer;

fn main() {

  let mut lexer = lexer::Lexer {};

  let mut counter = 0;

  let args : Vec<String> = env::args().collect();
  
  // take a slice of the input args vector referring to the input program
  let program_name = &args[1];
  
  // test program
  let mut program_file = File::open(program_name).expect("Could not open file");
  let mut program = String::new();
  
  program_file.read_to_string(&mut program).expect("Could not read file");
  
  // should have 51 tokens
  //let program = String::from(" program procedure global   ??? variable begin end is type integer float string bool enum if then else for return not true false . ; ( ) , { } - & + < > <= >= == != * / [ ] | := abcdef 1234 898.99 \"stringgoeshere\" \"fancie$t  string_g0es\n\nhere\t\"");
  
  // should have 3 tokens
  //let program = String::from("abc/**this is /*///*a***/*/ doc*/+bcd");
  
  let mut program_chars = program.chars().peekable();

  while let Some(tok) = lexer.next_tok(&mut program_chars) {
  
    println!("got token with chars: '{}' ({})", tok.chars, counter);

    counter += 1;
  }
    
  println!("num tokens: {}", counter);
  
}