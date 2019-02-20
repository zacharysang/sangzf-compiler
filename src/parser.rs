use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;

use crate::tokenize::lexable::Lexable;
use crate::lexer::Lexer;
use crate::tokenize::token::Token;
use crate::tokenize::token::TokenEntry;

pub struct Parser<'a> {
  pub lexer: Peekable<Lexer<'a>>,
  pub symbol_table_chain: Vec<HashMap<String, TokenEntry>>
}

impl <'a>Parser<'a> {
  pub fn new(program: Peekable<Chars<'a>>) -> Self {
    return Parser {lexer: Lexer::new(program).peekable(), symbol_table_chain: vec![]};
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
          if let Token::Period(_) = &tok_entry.tok_type {
            // consume period
            self.lexer.next();
            
            // Check that this is the end of the file
            if let None = self.lexer.peek() {
              println!("Program parsed successfully.");
              self.lexer.next();
              return true;
            } else if let Some(tok_entry) = self.lexer.peek() {
              // unexpected token after end of program
              Self::err_unexpected_tok("<end of program>", &tok_entry.chars);
            }
            
          } else {
            Self::err_unexpected_tok(".", &tok_entry.chars);
          }
        } else {
          Self::err_unexpected_end();
        }
      }
    }
    
    return false;
    
  }
  
  pub fn program_header(&mut self) -> bool {
    
    // tok_entry is a borrowed value so it will not be able to be moved
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::ProgramKW(_) = &tok_entry.tok_type {
        self.lexer.next();
        
        if let Some(tok_entry) = self.lexer.peek() {
          if let Token::Identifier(_) = &tok_entry.tok_type {
            self.lexer.next();
            
            if let Some(tok_entry) = self.lexer.peek() {
              if let Token::IsKW(_) = &tok_entry.tok_type {
                self.lexer.next();
                return true;
              } else {
                Self::err_unexpected_tok("is", &tok_entry.chars);
              }
            } else {
              Self::err_unexpected_end();
            }
            
          } else {
            Self::err_unexpected_tok("<identifier>", &tok_entry.chars);
          }
        } else {
          Self::err_unexpected_end();
        }
        
      } else {
        Self::err_unexpected_tok("program", &tok_entry.chars);
      }
    } else {
      Self::err_unexpected_end();
    }
  
    return false;
  }
  
  pub fn program_body(&mut self) -> bool {
  
    // while next token is in First(declaration), read in a declaration w/ semicolon terminator 
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        // lookahead to see if we should parse a declaration
        // ie: if next token is in First(declaration)
        let mut is_declaration = false;
        match &tok_entry.tok_type {
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => { is_declaration = true; },
          _ => break
        }
        
        
        if is_declaration {
          // parse a declaration (declarations are optional here so no need to check success status)
          self.declaration();
          
          self.semicolon();
          
        } else {
          break;
        }
        
      } else {
        Self::err_unexpected_end();
        break;
      }
    }
    
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::BeginKW(_) = &tok_entry.tok_type {
        self.lexer.next();
        
        // while next token is in First(statement), read in statement w/ semicolon terminator
        loop {
          if let Some(tok_entry) = self.lexer.peek() {
            
            let mut is_statement = false;
            match &tok_entry.tok_type {
              Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => { is_statement = true; },
              _ => break
            }
            
            if is_statement {
              self.statement();
              
              self.semicolon();
              
            } else {
              break;
            }
            
          } else {
            Self::err_unexpected_end();
            break;
          }
        }
        
        // parse the end kw
        if let Some(tok_entry) = self.lexer.peek() {
          if let Token::EndKW(_) = &tok_entry.tok_type {
            self.lexer.next();
            
            if let Some(tok_entry) = self.lexer.peek() {
              if let Token::ProgramKW(_) = &tok_entry.tok_type {
                self.lexer.next();
                
                return true;
              } else {
                Self::err_unexpected_tok("program", &tok_entry.chars);
              }
            } else {
              Self::err_unexpected_end();
            }
            
          } else {
            Self::err_unexpected_tok("end", &tok_entry.chars);
          }
        } else {
          Self::err_unexpected_end();
        }
        
      } else {
        Self::err_unexpected_tok("begin", &tok_entry.chars);
      }
    } else {
      Self::err_unexpected_end();
    }
  
    return false;
  }
  
  pub fn declaration(&mut self) -> bool {
  
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::GlobalKW(_) = &tok_entry.tok_type {
        self.lexer.next();
        
        // bring the global symbol table to the surface
        
      }
    }
    
    let mut got_declaration = false;
    if let Some(tok_entry) = self.lexer.peek() {
      match tok_entry.tok_type {
        Token::ProcedureKW(_) => { got_declaration = self.procedure_declaration(); },
        Token::VariableKW(_) => { got_declaration = self.variable_declaration(); },
        Token::TypeKW(_) => { got_declaration = self.variable_declaration(); },
        _ => { Self::err_unexpected_tok("(procedure|variable|type)", &tok_entry.chars); }
      }
    } else {
      Self::err_unexpected_end();
    }
    
    if got_declaration {
      return true;
    }
  
    return false;
  }
  
  pub fn procedure_declaration(&mut self) -> bool {
    return self.procedure_header() && self.procedure_body();
  }
  
  pub fn procedure_header(&mut self) -> bool {
  
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::ProcedureKW(_) = &tok_entry.tok_type {
        self.lexer.next();
        
        if let Some(tok_entry) = self.lexer.peek() {
          if let Token::Identifier(_) = &tok_entry.tok_type {
            self.lexer.next();
            
            if let Some(tok_entry) = self.lexer.peek() {
              if let Token::Colon(_) = &tok_entry.tok_type {
                self.lexer.next();
              
                if self.type_mark() {
                
                  if let Some(tok_entry) = self.lexer.peek() {
                    if let Token::LParen(_) = &tok_entry.tok_type {
                    
                      self.lexer.next();
                      
                      // read optional parameter list
                      self.parameter_list();
                      
                      if let Some(tok_entry) = self.lexer.peek() {
                        if let Token::RParen(_) = &tok_entry.tok_type {
                          self.lexer.next();
                        
                          return true;
                        } else {
                          Self::err_unexpected_tok(")", &tok_entry.chars);
                        }
                      } else {
                        Self::err_unexpected_end();
                      }
                      
                    } else {
                      Self::err_unexpected_tok("(", &tok_entry.chars);
                    }
                  } else {
                    Self::err_unexpected_end();
                  }
                  
                }
              } else {
                Self::err_unexpected_tok(":", &tok_entry.chars);
              }
            } else {
              Self::err_unexpected_end();
            }
            
          } else {
            Self::err_unexpected_tok("<identifier>", &tok_entry.chars);
          }
        } else {
          Self::err_unexpected_end();
        }
        
      } else {
        Self::err_unexpected_tok("procedure", &tok_entry.chars);
      }
    } else {
      Self::err_unexpected_end();
    }
  
    return true;
  }
  
  pub fn type_mark(&mut self) -> bool {
    return true;
  }
  
  pub fn parameter_list(&mut self) -> bool {
    return true;
  }
  
  pub fn procedure_body(&mut self) -> bool {
    return true;
  }
  
  pub fn variable_declaration(&mut self) -> bool {
    return true;
  }
  
  pub fn type_declaration(&mut self) -> bool {
    return true;
  }
  
  pub fn statement(&mut self) -> bool {
    return true;
  }
  
  pub fn semicolon(&mut self) -> bool {
    if let Some(tok_entry) = self.lexer.peek() {
      if let Token::Semicolon(_) = &tok_entry.tok_type {
        self.lexer.next();
        
        return true;
      } else {
        Self::err_unexpected_tok(";", &tok_entry.chars);
      }
    } else {
      Self::err_unexpected_end();
    }
    
    return false;
  }
  
  pub fn err_unexpected_tok(expected: &str, actual: &str) {
    println!("Unexpected token - Expected: '{}', got: '{}'", expected, actual);
  }
  
  pub fn err_unexpected_end() {
    println!("Unexpected end of program.");
  }
  
}

