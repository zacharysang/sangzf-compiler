use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;
use std::mem;

use crate::tokenize::lexable::Lexable;
use crate::lexer::Lexer;
use crate::tokenize::token::Token;
use crate::tokenize::token::TokenEntry;

use crate::tokens;

pub struct Parser<'a> {
  pub lexer: Peekable<Lexer<'a>>,
  pub symbol_table_chain: Vec<HashMap<String, TokenEntry>>,
  pub errors: Vec<ParserResult>
}

impl <'a>Parser<'a> {
  pub fn new(program: Peekable<Chars<'a>>) -> Self {
    return Parser {lexer: Lexer::new(program).peekable(), symbol_table_chain: vec![], errors: vec![]};
  }
  
  /* 
    Below are functions for different non-terminals
    Each function operates on an instance of the parser, processes the parse rule, and returns the success state
  */
  
  pub fn program(&mut self) -> ParserResult {
    
    let program_header = self.program_header();
    if let ParserResult::Success = program_header {
    
      let program_body = self.program_body();
      if let ParserResult::Success = program_body {
      
        // Check for terminating period
        let period = self.parse_tok(tokens::period::Period::start());
        if let ParserResult::Success = period {
          
          // Check that this is the end of the file
          if let None = self.lexer.peek() {
            println!("Program parsed successfully.");
            self.lexer.next();
            return ParserResult::Success;
          } else if let Some(tok_entry) = self.lexer.peek() {
            // unexpected token after end of program
            return ParserResult::ErrUnexpectedTok {expected: String::from("<end of program>"), actual: String::from(&tok_entry.chars[..])};
          }
          
        } else {
          period.print();
        }
        
      } else {
        program_body.print();
      }
      
    } else {
      program_header.print();
    }
    
    return ParserResult::Error;
    
  }
  
  pub fn program_header(&mut self) -> ParserResult {
    
    // tok_entry is a borrowed value so it will not be able to be moved
    let program_kw = self.parse_tok(tokens::program_kw::ProgramKW::start());
    if let ParserResult::Success = program_kw {
    
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success = identifier {
      
        let is_kw = self.parse_tok(tokens::is_kw::IsKW::start());
        if let ParserResult::Success = is_kw {
          return ParserResult::Success;
        } else {
          is_kw.print();
        }
        
      } else {
        identifier.print();
      }
    } else {
      program_kw.print();
    }
    
    return ParserResult::Error;
    
  }
  
  pub fn program_body(&mut self) -> ParserResult {
  
    // while next token is in First(declaration), read in a declaration w/ semicolon terminator 
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        match &tok_entry.tok_type {
          // these tokens are in First(declaration). Parse the declaration and a terminating semicolon
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => {
            if let ParserResult::Success = self.declaration() {
              self.parse_tok(tokens::semicolon::Semicolon::start());
            }
          },
          _ => break
        }
      }
    }
    
    let begin_kw = self.parse_tok(tokens::begin_kw::BeginKW::start());
    if let ParserResult::Success = begin_kw {
      // while next token is in First(statement), read in statement w/ semicolon terminator
      loop {
        if let Some(tok_entry) = self.lexer.peek() {
          match &tok_entry.tok_type {
            Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => {
              // if able to parse a statement, parse a terminating semicolon
              if let ParserResult::Success = self.statement() {
                self.parse_tok(tokens::semicolon::Semicolon::start());
              }
            },
            _ => break
          }
          
        } else {
          break;
        }
      }
      
      // parse the end kw
      let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
      if let ParserResult::Success = end_kw {
        let program_kw = self.parse_tok(tokens::program_kw::ProgramKW::start());
        if let ParserResult::Success = program_kw {
          return ParserResult::Success;
        }
      } else {
        end_kw.print();
      }
    } else {
      begin_kw.print();
    }
  
    return ParserResult::Error;
  }
  
  pub fn declaration(&mut self) -> ParserResult {
  
    if let ParserResult::Success = self.parse_tok(tokens::global_kw::GlobalKW::start()) {
      // bring the global symbol table to focus
    }
    
    if let Some(tok_entry) = self.lexer.peek() {
    
      match &tok_entry.tok_type {
        Token::ProcedureKW(_tok) => { return self.procedure_declaration(); },
        Token::VariableKW(_tok) => {return self.variable_declaration(); },
        Token::TypeKW(_tok) => { return self.type_declaration(); },
        _ => { return ParserResult::ErrUnexpectedTok {expected: String::from("(procedure|variable|type)"), actual: String::from(&tok_entry.chars[..])}; }
      }
      
    } else {
      return ParserResult::ErrUnexpectedEnd;
    }
  }
  
  pub fn procedure_declaration(&mut self) -> ParserResult {
    let procedure_header = self.procedure_header();
    if let ParserResult::Success = procedure_header {
      let procedure_body = self.procedure_body();
      if let ParserResult::Success = procedure_body {
        return ParserResult::Success;
      } else {
        procedure_body.print();
      }
    } else {
      procedure_header.print();
    }
    
    return ParserResult::Error;
  }
  
  pub fn procedure_header(&mut self) -> ParserResult {
    let procedure_kw = self.parse_tok(tokens::procedure_kw::ProcedureKW::start());
    if let ParserResult::Success = procedure_kw {
    
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success = identifier {
        let colon = self.parse_tok(tokens::colon::Colon::start());
        if let ParserResult::Success = colon {
          let type_mark = self.type_mark();
          if let ParserResult::Success = type_mark {
            let l_paren = self.parse_tok(tokens::parens::LParen::start());
            if let ParserResult::Success = l_paren {
            
              // read optional parameter list
              self.parameter_list();
              
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success = r_paren {
                return ParserResult::Success;
              } else {
                r_paren.print();
              }
            } else {
              l_paren.print();
            }
          } else {
             type_mark.print();
          }
        } else {
          colon.print();
        }
      } else {
        identifier.print();
      }
    } else {
      procedure_kw.print();
    }
  
    return ParserResult::Error;
  }
  
  pub fn type_mark(&mut self) -> ParserResult {
  
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = peek_tok {
      match &tok_entry.tok_type {
        Token::IntegerKW(_) => { self.lexer.next(); return ParserResult::Success; },
        Token::FloatKW(_) => { self.lexer.next(); return ParserResult::Success; },
        Token::StringKW(_) => { self.lexer.next(); return ParserResult::Success; },
        Token::BoolKW(_) => { self.lexer.next(); return ParserResult::Success; },
        Token::Identifier(_) => { self.lexer.next(); return ParserResult::Success; },
        Token::EnumKW(_) => {
          self.lexer.next();
          
          let l_brace = self.parse_tok(tokens::braces::LBrace::start());
          if let ParserResult::Success = l_brace {
            let identifier = self.parse_tok(tokens::identifier::Identifier::start());
            if let ParserResult::Success = identifier {
            
              loop {
                // optionally parse additional identifiers (delimited by comma)
                if let ParserResult::Success = self.parse_tok(tokens::comma::Comma::start()) {
                  self.parse_tok(tokens::identifier::Identifier::start());
                } else {
                  break;
                }
              }
              
              let r_brace = self.parse_tok(tokens::braces::RBrace::start());
              if let ParserResult::Success = r_brace {
                return ParserResult::Success;
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
        _ => { return ParserResult::ErrUnexpectedTok{expected: String::from("<some_type_kw>"), actual: String::from(&tok_entry.chars[..])}; }
      }
    } else {
      return ParserResult::ErrUnexpectedEnd;
    }
  }
  
  pub fn parameter_list(&mut self) -> ParserResult {
    
    let parameter = self.parameter();
    if let ParserResult::Success = parameter {
      
      // optionally parse another parameter list (delimited by comma)
      let comma = self.parse_tok(tokens::comma::Comma::start());
      if let ParserResult::Success = comma {
        // call recursively to parse the rest of the list
        return self.parameter_list();
      } else {
        return ParserResult::Success;
      }
      
    } else {
      parameter.print();
      return parameter;
    }
    
  }
  
  pub fn parameter(&mut self) -> ParserResult {
    return ParserResult::Success;
  }
  
  pub fn procedure_body(&mut self) -> ParserResult {
    return ParserResult::Success;
  }
  
  pub fn variable_declaration(&mut self) -> ParserResult {
    return ParserResult::Success;
  }
  
  pub fn type_declaration(&mut self) -> ParserResult {
    return ParserResult::Success;
  }
  
  pub fn statement(&mut self) -> ParserResult {
    return ParserResult::Success;
  }
  
  // TODO get this to use generics so that a dummy target token doesn't need to be passed in
  pub fn parse_tok(&mut self, target: Token) -> ParserResult {
    if let Some(tok_entry) = self.lexer.peek() {
      if mem::discriminant(&tok_entry.tok_type) == mem::discriminant(&target) {
        self.lexer.next();
      
        return ParserResult::Success;
      } else {
        return ParserResult::ErrUnexpectedTok {expected: String::from(target.get_example()), actual: String::from(&tok_entry.chars[..])};
      }
    } else {
      return ParserResult::ErrUnexpectedEnd
    }
  }
}

pub enum ParserResult {
  ErrUnexpectedEnd,
  ErrUnexpectedTok{ expected: String, actual: String },
  Success,
  Error
}

impl ParserResult {
  pub fn print(&self) {
    match self {
      ParserResult::ErrUnexpectedEnd => { println!("Unexpected end of program."); },
      ParserResult::ErrUnexpectedTok{ expected, actual } => { println!("Unexpected token - Expected: '{}', got: '{}'", expected, actual); },
      ParserResult::Success => { println!("Success"); },
      ParserResult::Error => { println!("Unknown error"); }
    }
  }
}
