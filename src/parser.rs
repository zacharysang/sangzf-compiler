use std::iter::Peekable;
use std::str::Chars;

use crate::tokenize::lexable::Lexable;
use crate::lexer::Lexer;
use crate::tokenize::token::Token;

pub struct Parser<'a> {
  pub lexer: Peekable<Lexer<'a>>
}

impl <'a>Parser<'a> {
  pub fn new(program: Peekable<Chars<'a>>) -> Self {
    return Parser {lexer: Lexer::new(program).peekable()};
  }
  
  /* 
    Below are functions for different non-terminals
    Each function operates on an instance of the parser, processes the parse rule, and returns the success state
  */
  
  pub fn program(&mut self) -> bool {
    if self.program_header() {
      if self.program_body() {
        // Check for terminating period
        if let Some(tok_entry) = self.lexer.peek() {
          if let Token::Period(_tok) = &tok_entry.tok_type {
            // consume
            self.lexer.next();
            
            // Check that this is the end of the file
            if let None = self.lexer.peek() {
              println!("Program parsed successfully.");
              self.lexer.next();
              return true;
            } else if let Some(tok_entry) = self.lexer.peek() {
              // unexpected token after end of program
              println!("Unexpected token '{}' after end of program", &tok_entry.chars);
            }
            
          } else {
            println!("Unexpected token - Expected '.' got '{}'", &tok_entry.chars);
          }
        } else {
          println!("Missing '.' after program body");
        }
      }
    }
    
    return false;
    
  }
  
  pub fn program_header(&mut self) -> bool {
    
    // tok_entry is a borrowed value so it will not be able to be moved
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::ProgramKW(_tok) = &tok_entry.tok_type {
        self.lexer.next();
        
        if let Some(tok_entry) = self.lexer.peek() {
          if let Token::Identifier(_tok) = &tok_entry.tok_type {
            self.lexer.next();
            
            if let Some(tok_entry) = self.lexer.peek() {
              if let Token::IsKW(_tok) = &tok_entry.tok_type {
                self.lexer.next();
                return true;
              } else {
                println!("Unexpected token - Expected 'is', got '{}'", &tok_entry.chars);
              }
            } else {
              println!("Unexpected end of program");
            }
            
          } else {
            println!("Unexpected token - Expected '<identifier>', got '{}'", &tok_entry.chars);
          }
        } else {
          println!("Unexpected end of program");
        }
        
      } else {
        println!("Unexpected token - Expected 'program' got '{}'", &tok_entry.chars);
      }
    } else {
      println!("Unexpected end of program");
    }
  
    return true;
  }
  
  pub fn program_body(&mut self) -> bool {
  
    
  
    return true;
  }
  
  
}