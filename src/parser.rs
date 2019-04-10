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
    let lexer = Lexer::new(program);
    return Parser {lexer: lexer.peekable(), symbol_table_chain: vec![], errors: vec![]};
  }
  
  
  /* 
    Below are functions for different parse rules
    Each function operates on an instance of the parser, processes the parse rule, and returns the success state or the reason why the parse failed
    On a failed parse, an unknown number of tokens will be consumed (may have partially consumed)
    For 'parse_tok', if unsuccessful, no tokens will have been consumed
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
            println!("Program parsed.");
            self.lexer.next();
            return ParserResult::Success;
          } else if let Some(tok_entry) = self.lexer.peek() {
            // unexpected token after end of program
            let result = ParserResult::ErrUnexpectedTok {expected: String::from("<end of program>"), actual: String::from(&tok_entry.chars[..])};
            
            result.print();
            
            return result;
          }
          
        } else { period.print(); }
      } else { program_body.print(); }
    } else { program_header.print(); }
    
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
        
      } else { identifier.print(); }
    } else { program_kw.print(); }
    
    return ParserResult::Error;
    
  }
  
  pub fn program_body(&mut self) -> ParserResult {
    // while next token is in First(declaration), read in a declaration w/ semicolon terminator 
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        match &tok_entry.tok_type {
          // these tokens are in First(declaration). Parse the declaration and a terminating semicolon
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => {
            let declaration = self.declaration();
            if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = &declaration {
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
    if let ParserResult::Success = begin_kw {
      // while next token is in First(statement), read in statement w/ semicolon terminator
      loop {
        if let Some(tok_entry) = self.lexer.peek() {
          match &tok_entry.tok_type {
            Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => {
              // if able to parse a statement, parse a terminating semicolon
              let statement = self.statement();
              if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = statement {
                statement.print();
              }
              
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
      if let ParserResult::Success = end_kw {
        let program_kw = self.parse_tok(tokens::program_kw::ProgramKW::start());
        if let ParserResult::Success = program_kw {
          return ParserResult::Success;
        }
      } else { end_kw.print(); }
    } else { begin_kw.print(); }
  
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
      } else { procedure_body.print(); }
    } else { procedure_header.print(); }
    
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
              let next_tok = self.lexer.peek();
              if let Some(tok_entry) = next_tok {
                if let Token::VariableKW(_) = &tok_entry.tok_type {
                  self.parameter_list();
                }
              }
              
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success = r_paren {
                return ParserResult::Success;
              } else { r_paren.print(); }
            } else { l_paren.print(); }
          } else { type_mark.print(); }
        } else { colon.print(); }
      } else { identifier.print(); }
    } else { procedure_kw.print(); }
  
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
    } else { return ParserResult::ErrUnexpectedEnd; }
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
    return self.variable_declaration();
  }
  
  pub fn procedure_body(&mut self) -> ParserResult {
  
    // TODO break this out to its own function
    // parse an optional number of declarations delimited by semicolon
    loop {
      if let Some(tok_entry) = self.lexer.peek() {

        match &tok_entry.tok_type {
          // these tokens are in First(declaration). Parse the declaration and a terminating semicolon
          Token::GlobalKW(_) | Token::ProcedureKW(_) | Token::VariableKW(_) | Token::TypeKW(_) => {
            let declaration = self.declaration();
            if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = declaration {
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
    if let ParserResult::Success = begin_kw {
      // parse an optional number of statements
      // TODO break this out to its own function
      loop {
        if let Some(tok_entry) = self.lexer.peek() {
          match &tok_entry.tok_type {
            Token::Identifier(_) | Token::IfKW(_) | Token::ForKW(_) | Token::ReturnKW(_) => {
              let statement = self.statement();
              if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = statement {
                statement.print();
              }
              
              self.resync();
            },
            _ => break
          }
          
        } else {
          break;
        }
      }
      
      let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
      if let ParserResult::Success = end_kw {
        let procedure_kw = self.parse_tok(tokens::procedure_kw::ProcedureKW::start());
        if let ParserResult::Success = procedure_kw {
          return ParserResult::Success;
        } else { return procedure_kw; }
      } else { return end_kw; }
    } else { return begin_kw; }
  }
  
  pub fn variable_declaration(&mut self) -> ParserResult {
    
    let variable_kw = self.parse_tok(tokens::variable_kw::VariableKW::start());
    if let ParserResult::Success = variable_kw {
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success = identifier {
        let colon = self.parse_tok(tokens::colon::Colon::start());
        if let ParserResult::Success = colon {
          let type_mark = self.type_mark();
          if let ParserResult::Success = type_mark {
            
            // optionally parse a bound
            let l_bracket = self.parse_tok(tokens::brackets::LBracket::start());
            if let ParserResult::Success = l_bracket {
              let bound = self.bound();
              if let ParserResult::Success = bound {
                let r_bracket = self.parse_tok(tokens::brackets::RBracket::start());
                if let ParserResult::Success = r_bracket {
                  return ParserResult::Success;
                } else {
                  return r_bracket;
                }
              } else {
                return bound;
              }
            }
            
            return ParserResult::Success;
            
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
  
  pub fn type_declaration(&mut self) -> ParserResult {
    let type_kw = self.parse_tok(tokens::type_kw::TypeKW::start());
    if let ParserResult::Success = type_kw {
      let identifier = self.parse_tok(tokens::identifier::Identifier::start());
      if let ParserResult::Success = identifier {
        let is_kw = self.parse_tok(tokens::is_kw::IsKW::start());
        if let ParserResult::Success = is_kw {
          let type_mark = self.type_mark();
          if let ParserResult::Success = type_mark {
            return ParserResult::Success;
          } else { return type_mark; }
        } else { return is_kw; }
      } else { return identifier; }
    } else { return type_kw; }
    
  }
  
  pub fn statement(&mut self) -> ParserResult {
    
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = peek_tok {
      match &tok_entry.tok_type {
        Token::Identifier(_) => { return self.assignment_statement(); },
        Token::IfKW(_) => { return self.if_statement(); },
        Token::ForKW(_) => { return self.loop_statement(); },
        Token::ReturnKW(_) => { return self.return_statement(); },
        _ => { return ParserResult::ErrUnexpectedTok {expected: String::from("(<identifier>|if|for|return)"), actual: String::from(&tok_entry.chars[..])} }
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  pub fn procedure_call_w_identifier(&mut self, identifier: ParserResult) -> ParserResult {
    if let ParserResult::Success = identifier {
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success = l_paren {
        
        // parse optional argument list
        self.argument_list();
        
        let r_paren = self.parse_tok(tokens::parens::RParen::start());
        if let ParserResult::Success = r_paren {
          return ParserResult::Success;
        } else { return r_paren; }
      } else { return l_paren; }
    } else { return identifier; }
  }
  
  // this is currently unused since procedure calls currently only occur ambiguously with names (in the factor parse rule)
  pub fn procedure_call(&mut self) -> ParserResult {
  
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    return self.procedure_call_w_identifier(identifier);
    
  }
  
  pub fn name_w_identifier(&mut self, identifier: ParserResult) -> ParserResult {
    if let ParserResult::Success = identifier {
    
      // optionally parse square bracket
      let peek_tok = self.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        // if next up is an LBracket, commit to parsing this optional portion
        if let Token::LBracket(_) = &tok_entry.tok_type {
          // consume the l_bracket
          self.lexer.next();
          
          let expression = self.expression();
          if let ParserResult::Success = expression {
            let r_bracket = self.parse_tok(tokens::brackets::RBracket::start());
            if let ParserResult::Success = r_bracket {
              return ParserResult::Success;
            } else {
              return r_bracket;
            }
          } else {
            return expression;
          }
        }
      }
      
      return ParserResult::Success;
      
    } else { return identifier; }
  }
  
  pub fn name(&mut self) -> ParserResult {
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    return self.name_w_identifier(identifier);
  }
  
  pub fn term(&mut self) -> ParserResult {
  
    // define function for factored parse rule
    fn _term<'a>(slf: &mut Parser<'a>) -> ParserResult {
      // accept either a '*' or '/'
      let asterisk = slf.parse_tok(tokens::asterisk::Asterisk::start());
      if let ParserResult::Success = asterisk {
        let factor = slf.factor();
        if let ParserResult::Success = factor {
          return _term(slf);
        } else {
          return factor;
        }
      }
      
      let slash = slf.parse_tok(tokens::slash::Slash::start());
      if let ParserResult::Success = slash {
        let factor = slf.factor();
        if let ParserResult::Success = factor {
          return _term(slf);
        } else {
          return factor;
        }
      }
      
      // allow _term to parse nothing
      return ParserResult::Success;
      
    }
  
    // read bottomed-out factor rule
    let factor = self.factor();
    if let ParserResult::Success = factor {
      return _term(self);
    } else { return factor; }
    
  }
  
  pub fn relation(&mut self) -> ParserResult {
    fn _relation<'a>(slf: &mut Parser<'a>) -> ParserResult {
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::LT(_) => {
            let lt = slf.parse_tok(tokens::lt::LT::start());
            if let ParserResult::Success = lt {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else {
                return term;
              }
            } else {
              return lt;
            }
          },
          Token::GTE(_) => {
            let gte = slf.parse_tok(tokens::gte::GTE::start());
            if let ParserResult::Success = gte {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else {
                return term;
              }
            } else {
              return gte;
            }
          },
          Token::LTE(_) => {
            let lte = slf.parse_tok(tokens::lte::LTE::start());
            if let ParserResult::Success = lte {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else {
                return term;
              }
            } else {
              return lte;
            }
          },
          Token::GT(_) => {
            let gt = slf.parse_tok(tokens::gt::GT::start());
            if let ParserResult::Success = gt {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else {
                return term;
              }
            } else {
              return gt;
            }
          },
          Token::EQ(_) => {
            let eq = slf.parse_tok(tokens::eq::EQ::start());
            if let ParserResult::Success = eq {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else { return term; }
            } else { return eq; }
          },
          Token::NEQ(_) => {
            let neq = slf.parse_tok(tokens::neq::NEQ::start());
            if let ParserResult::Success = neq {
              let term = slf.term();
              if let ParserResult::Success = term {
                return _relation(slf);
              } else { return term; }
            } else { return neq; }
          }
          _ => { 
            // allow nothing to be parsed (allow lambda production)
            // this is the base case for this recursive function
            // will keep recursing until next token is not a comparison (<, >, etc.)
            return ParserResult::Success; 
          }
        }
      } else {
        // similar to catch-all, allow nothing to be parsed
        return ParserResult::Success;
      }
    }
    
    let term = self.term();
    if let ParserResult::Success = term {
      return _relation(self);
    } else {
      return term;
    }
    
  }
  
  pub fn arith_op(&mut self) -> ParserResult {
    fn _arith_op(slf: &mut Parser) -> ParserResult {
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::Plus(_) => {
            let plus = slf.parse_tok(tokens::plus::Plus::start());
            if let ParserResult::Success = plus {
              let relation = slf.relation();
              if let ParserResult::Success = relation {
                return _arith_op(slf);
              } else {
                return relation;
              }
            } else {
              return plus;
            }
          },
          Token::Dash(_) => {
            let dash = slf.parse_tok(tokens::dash::Dash::start());
            if let ParserResult::Success = dash {
              let relation = slf.relation();
              if let ParserResult::Success = relation {
                return _arith_op(slf);
              } else {
                return relation;
              }
            } else {
              return dash;
            }
          },
          _ => {
            // base case: allow nothing to be parsed if '+' and '-' not found
            return ParserResult::Success;
          }
        }
      } else {
        // base case
        return ParserResult::Success;
      }
    }
    
    // parse the initial relation where the recursion bottoms out
    let relation = self.relation();
    if let ParserResult::Success = relation {
      return _arith_op(self);
    } else { return relation; }
  }
  
  pub fn expression(&mut self) -> ParserResult {
    fn _expression(slf: &mut Parser) -> ParserResult {
      let peek_tok = slf.lexer.peek();
      if let Some(tok_entry) = peek_tok {
        match &tok_entry.tok_type {
          Token::Ampersand(_) => {
            let ampersand = slf.parse_tok(tokens::ampersand::Ampersand::start());
            if let ParserResult::Success = ampersand {
              let arith_op = slf.arith_op();
              if let ParserResult::Success = arith_op {
                return _expression(slf);
              } else {
                return arith_op;
              }
            } else {
              return ampersand;
            }
          },
          Token::Pipe(_) => {
            let pipe = slf.parse_tok(tokens::pipe::Pipe::start());
            if let ParserResult::Success = pipe {
              let arith_op = slf.arith_op();
              if let ParserResult::Success = arith_op {
                return _expression(slf);
              } else {
                return arith_op;
              }
            } else {
              return pipe;
            }
          },
          _ => {
            // base case: if non-matching token is hit, do not parse (lambda-production)
            return ParserResult::Success;
          }
        }
      } else {
        // base case: allow nothing to be parsed
        return ParserResult::Success;
      }
    }
    
    // optionally parse a 'not' kw
    self.parse_tok(tokens::not_kw::NotKW::start());
    
    let arith_op = self.arith_op();
    if let ParserResult::Success = arith_op {
      return _expression(self);
    } else { return arith_op; }
  }
  
  pub fn argument_list(&mut self) -> ParserResult {
    let expression = self.expression();
    if let ParserResult::Success = expression {
      // optionally parse the rest
      let comma = self.parse_tok(tokens::comma::Comma::start());
      if let ParserResult::Success = comma {
        return self.argument_list();
      }
      
      return ParserResult::Success;
    } else { return expression; }
  }
  
  pub fn assignment_statement(&mut self) -> ParserResult {
    let destination = self.destination();
    if let ParserResult::Success = destination {
      let assign = self.parse_tok(tokens::assign::Assign::start());
      if let ParserResult::Success = assign {
        let expression = self.expression();
        if let ParserResult::Success = expression {
          return ParserResult::Success;
        } else { return expression; }
      } else { return assign; }
    } else { return destination; }
  }
  
  pub fn destination(&mut self) -> ParserResult {
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    if let ParserResult::Success = identifier {
      // optionally parse an index to this value
      let l_bracket = self.parse_tok(tokens::brackets::LBracket::start());
      if let ParserResult::Success = l_bracket {
        let expression = self.expression();
        if let ParserResult::Success = expression {
          return self.parse_tok(tokens::brackets::RBracket::start());
        } else {
          return expression;
        }
      }
      
      return ParserResult::Success;
      
    } else { return identifier; }
  }
  
  pub fn if_statement(&mut self) -> ParserResult {
    let if_kw = self.parse_tok(tokens::if_kw::IfKW::start());
    if let ParserResult::Success = if_kw {
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success = l_paren {
        let expression = self.expression();
        if let ParserResult::Success = expression {
          let r_paren = self.parse_tok(tokens::parens::RParen::start());
          if let ParserResult::Success = r_paren {
            let then_kw = self.parse_tok(tokens::then_kw::ThenKW::start());
            if let ParserResult::Success = then_kw {
              
              // parse an arbitrary number of statements delimited by ';'
              loop {
                if let Some(tok_entry) = self.lexer.peek() {
                  match tok_entry.tok_type {
                    Token::ElseKW(_) | Token::EndKW(_) => break,
                    _ => {
                      let statement = self.statement();
                      if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = statement {
                        statement.print();
                      } 
                      
                      self.resync();
                    }
                  }
                } else {
                  break;
                }
              }
              
              // optionally parse else statement
              let else_kw = self.parse_tok(tokens::else_kw::ElseKW::start());
              if let ParserResult::Success = else_kw {
                // parse an arbitrary number of statements delimited by ';'
                loop {
                  if let Some(tok_entry) = self.lexer.peek() {
                    match tok_entry.tok_type {
                      Token::EndKW(_) => break,
                      _ => {
                        let statement = self.statement();
                        if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = statement {
                          statement.print();
                        } 
                        
                        self.resync();
                      }
                    }
                  } else {
                    break;
                  }
                }
              }
              
              let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
              if let ParserResult::Success = end_kw {
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
  
  pub fn loop_statement(&mut self) -> ParserResult {
    let for_kw = self.parse_tok(tokens::for_kw::ForKW::start());
    if let ParserResult::Success = for_kw {
      let l_paren = self.parse_tok(tokens::parens::LParen::start());
      if let ParserResult::Success = l_paren {
        let assignment_statement = self.assignment_statement();
        if let ParserResult::Success = assignment_statement {
          let semicolon = self.parse_tok(tokens::semicolon::Semicolon::start());
          if let ParserResult::Success = semicolon {
            let expression = self.expression();
            if let ParserResult::Success = expression {
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success = r_paren {
                
                // parse an arbitrary number of statements delimited by ';'
                loop {
                  if let Some(tok_entry) = self.lexer.peek() {
                    match tok_entry.tok_type {
                      Token::EndKW(_) => break,
                      _ => {
                        let statement = self.statement();
                        if let ParserResult::ErrUnexpectedEnd | ParserResult::ErrUnexpectedTok{..} | ParserResult::Error = statement {
                          statement.print();
                        }
                        self.resync();
                      }
                    }
                  } else {
                    break;
                  }
                }
                
                let end_kw = self.parse_tok(tokens::end_kw::EndKW::start());
                if let ParserResult::Success = end_kw {
                  return self.parse_tok(tokens::for_kw::ForKW::start());
                } else { return end_kw; }
              } else { return r_paren; }
            } else { return expression; }
          } else { return semicolon; }
        } else { return assignment_statement; }
      } else { return l_paren; }
    } else { return for_kw; }
  }
  
  pub fn return_statement(&mut self) -> ParserResult {
    let return_kw = self.parse_tok(tokens::return_kw::ReturnKW::start());
    if let ParserResult::Success = return_kw {
      return self.expression();
    } else { return return_kw; }
  }

  pub fn procedure_call_or_name(&mut self) -> ParserResult {
    // this could be a procedure call or a name based on the next token
    let identifier = self.parse_tok(tokens::identifier::Identifier::start());
    
    let peek_tok = self.lexer.peek();
    
    if let Some(tok_entry) = &peek_tok {
      if let Token::LParen(_) = &tok_entry.tok_type {
        return self.procedure_call_w_identifier(identifier);
      }
    }
    
    return self.name_w_identifier(identifier);
  }
  
  pub fn name_or_number(&mut self) -> ParserResult {
    
    // optionally parse a dash
    // TODO capture the value so it can be used when writing to the symbol table
    self.parse_tok(tokens::dash::Dash::start());
    
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = &peek_tok {
      match &tok_entry.tok_type {
        Token::Identifier(_) => { return self.name() },
        Token::Number(_) => { return self.parse_tok(tokens::number::Number::start()) },
        _ => { return ParserResult::ErrUnexpectedTok {expected: String::from("(<identifier>|<number>)"), actual: String::from(&tok_entry.chars[..])}; }
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  pub fn factor(&mut self) -> ParserResult {
    // peek at next token to decide what type of factor this will be
    let peek_tok = self.lexer.peek();
    if let Some(tok_entry) = &peek_tok {
      match &tok_entry.tok_type {
        Token::LParen(_) => {
          let l_paren = self.parse_tok(tokens::parens::LParen::start());
          if let ParserResult::Success = l_paren {
            let expression = self.expression();
            if let ParserResult::Success = expression {
              let r_paren = self.parse_tok(tokens::parens::RParen::start());
              if let ParserResult::Success = r_paren {
                return ParserResult::Success;
              } else { return r_paren; }
            } else { return expression; }
          } else { return l_paren; }
        },
        Token::Identifier(_) => { return self.procedure_call_or_name(); },
        Token::Dash(_) => {
          // this could be a name or number depending on the next token
          return self.name_or_number();
        },
        Token::String(_) => { return self.parse_tok(tokens::string::String::start()); },
        Token::Number(_) => { return self.parse_tok(tokens::number::Number::start()); }
        Token::TrueKW(_) => { return self.parse_tok(tokens::true_kw::TrueKW::start()); },
        Token::FalseKW(_) => { return self.parse_tok(tokens::false_kw::FalseKW::start()); },
        _ => { return ParserResult::ErrUnexpectedTok {expected: String::from("('('|<identifier>|'-'|<number>|<string>|true|false)"), actual: String::from(&tok_entry.chars[..])} }
      }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  // TODO get this to use generics so that a dummy target token doesn't need to be passed in
  pub fn parse_tok(&mut self, target: Token) -> ParserResult {
    if let Some(tok_entry) = self.lexer.peek() {
      // if the next token matches the target, consume and return success result
      if mem::discriminant(&tok_entry.tok_type) == mem::discriminant(&target) {
        self.lexer.next();
      
        return ParserResult::Success;
      } else { return ParserResult::ErrUnexpectedTok {expected: String::from(target.get_example()), actual: String::from(&tok_entry.chars[..])}; }
    } else { return ParserResult::ErrUnexpectedEnd; }
  }
  
  // consume tokens until a semicolon is hit, and then consume the semicolon
  pub fn resync(&mut self) {
    loop {
      let result = self.parse_tok(tokens::semicolon::Semicolon::start());
      
      match result {
        ParserResult::Success => break,
        ParserResult::ErrUnexpectedEnd => {
          result.print();
          break;
        },
        _ => {
          result.print();
          self.lexer.next();
        }
      }
    }
  }
  
}

// TODO add chaining behavior so that we can chain parse rules until a failure is reached. This would really simplify this code by flattening all these nested ifs
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
      ParserResult::Error => { println!("Unknown error"); },
      ParserResult::Success => { println!("Success"); }
    }
  }
}