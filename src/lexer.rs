use std::iter::Peekable;
use std::str::Chars;

// bring Token into scope so we can use its associated items
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;
use crate::tokenize::token::TokenEntry;
use crate::tokenize::char_group::CharGroup;

use crate::tokens;

pub struct Lexer<'a> {
  pub program: Peekable<Chars<'a>>,
  pub line_num: u32,
  pub errors: Vec<String>
}

impl <'a> Lexer<'a> {

  pub fn new(program: Peekable<Chars<'a>>) -> Self {
    return Lexer {program: program, line_num: 1, errors: vec![]};
  }
  
  fn all_tokens() -> Vec<Token> {
  
    // TODO put all these into a direct declaration rather than adding them all in
    let mut token_types = Vec::new();
    
    // catch all token type. Goes first so that liveness will be overwritten by other token types
    token_types.push(tokens::unknown::Unknown::start());
    
    token_types.push(tokens::period::Period::start());
    token_types.push(tokens::semicolon::Semicolon::start());
    token_types.push(tokens::colon::Colon::start());
    token_types.push(tokens::parens::LParen::start());
    token_types.push(tokens::parens::RParen::start());
    token_types.push(tokens::comma::Comma::start());
    token_types.push(tokens::braces::LBrace::start());
    token_types.push(tokens::braces::RBrace::start());
    token_types.push(tokens::dash::Dash::start());
    token_types.push(tokens::brackets::LBracket::start());
    token_types.push(tokens::brackets::RBracket::start());
    token_types.push(tokens::pipe::Pipe::start());
    token_types.push(tokens::ampersand::Ampersand::start());
    token_types.push(tokens::plus::Plus::start());
    token_types.push(tokens::lt::LT::start());
    token_types.push(tokens::gt::GT::start());
    token_types.push(tokens::lte::LTE::start());
    token_types.push(tokens::gte::GTE::start());
    token_types.push(tokens::eq::EQ::start());
    token_types.push(tokens::neq::NEQ::start());
    token_types.push(tokens::asterisk::Asterisk::start());
    token_types.push(tokens::slash::Slash::start());
    token_types.push(tokens::assign::Assign::start());
    
    token_types.push(tokens::identifier::Identifier::start());
    token_types.push(tokens::number::Number::start());
    token_types.push(tokens::string::String::start());
    
    // These must be last (except comments) so that they overwrite others
    token_types.push(tokens::program_kw::ProgramKW::start());
    token_types.push(tokens::begin_kw::BeginKW::start());
    token_types.push(tokens::end_kw::EndKW::start());
    token_types.push(tokens::is_kw::IsKW::start());
    token_types.push(tokens::global_kw::GlobalKW::start());
    token_types.push(tokens::procedure_kw::ProcedureKW::start());
    token_types.push(tokens::variable_kw::VariableKW::start());
    token_types.push(tokens::type_kw::TypeKW::start());
    token_types.push(tokens::integer_kw::IntegerKW::start());
    token_types.push(tokens::float_kw::FloatKW::start());
    token_types.push(tokens::string_kw::StringKW::start());
    token_types.push(tokens::bool_kw::BoolKW::start());
    token_types.push(tokens::enum_kw::EnumKW::start());
    token_types.push(tokens::if_kw::IfKW::start());
    token_types.push(tokens::then_kw::ThenKW::start());
    token_types.push(tokens::else_kw::ElseKW::start());
    token_types.push(tokens::for_kw::ForKW::start());
    token_types.push(tokens::return_kw::ReturnKW::start());
    token_types.push(tokens::not_kw::NotKW::start());
    token_types.push(tokens::true_kw::TrueKW::start());
    token_types.push(tokens::false_kw::FalseKW::start());
    
    // comments must go last to overwrite everything else
    token_types.push(tokens::line_comment::LineComment::start());
    
    // this currently breaks. check state machine..
    token_types.push(tokens::multiline_comment::MultilineComment::start());
    
    return token_types;
  }
  
}

impl <'a> Iterator for Lexer<'a> {
  type Item = TokenEntry;
  
  fn next(&mut self) -> Option<Self::Item> {
    
    // the value to eventually return
    let mut next_token = None;
  
    loop {  
    
      let mut is_comment = false;
      
      // make a collection with state for all token types
      let mut token_types = Lexer::all_tokens();

      // information about an acceptable token
      // (not accepted until last live token type)
      let mut acceptable_idx = None;
      let mut acceptable_chars = None;
      
      // check for end of file
      if let None = self.program.peek() {
        println!("EOF Reached");
        return None;
      }
      
      // ensure head of iterator is a non-ws
      while let Some(ch) = self.program.peek() {
        if !CharGroup::is_ws(*ch) {
          break;
        }
        
        if '\n' == *ch {
          self.line_num += 1;
        }
        
        self.program.next();
      }
      
      // while at more than one (disregarding 'unknown') token is alive (tok.state != None), keep consuming characters
      // if the token is acceptable, record it
      // when none are alive state, return accept token
      // if acceptable == None, error
      let mut alive = 2;
      while alive > 1 {
      
        alive = 0;
          
        // get value at the head of the iterator
        let curr_ch = self.program.peek();
      
        if let Some(ch) = curr_ch {
        
          for (i, token) in token_types.iter_mut().enumerate() {
          
            // advance curr type and ensure that it's still a valid state
            if let Some(state) = token.next(*ch) {
            
              alive += 1;
              
              // reassign state if acceptable
              if state.accept {
              
                // make a new Token that matches this element
                acceptable_idx = Some(i);
                
                // get a copy of the string
                acceptable_chars = Some(String::from(&state.chars[..]));
              }
            }
          }
        }
        
        // if alive will be true in the next iteration, advance iterator
        // or if head results in a live state, then consume this char
        // token 'unknown' if dead and none accepted (alive == 1 && acceptable_idx.is_none())
        // need to advance if alive or unknown (if unknown and we do not advance, will get stuck on the unknown token)
        if alive > 1 || acceptable_idx.is_none() {
          
          let curr_ch = self.program.next();
          
          if let Some(ch) = curr_ch {
            if '\n' == ch {
              self.line_num += 1;
            }
          }
          
        }
        
      }
      
      // After loop, setup return value
      if let Some(idx) = acceptable_idx {
      
        if let Some(chars) = acceptable_chars {
        
          let tok_type = token_types.remove(idx);
        
          // check if tok_type is a comment
          if let Token::LineComment(_tok) = &tok_type {
            is_comment = true;
          }
          if let Token::MultilineComment(_tok) = &tok_type {
            is_comment = true;
          }
        
          next_token = Some(TokenEntry { chars: chars, tok_type: tok_type });
        }
    
      } else {
      
        let caught_tok = token_types.remove(0);
        
        if let Some(state) = caught_tok.get_state() {
        
          let chars = &state.chars;
          
          // only report errors on non-zero tokens
          if chars.len() > 0 {
            self.errors.push(format!("Error! Unrecognized token, '{}' at line: {}", chars, self.line_num));
            next_token = Some(TokenEntry {chars: chars.to_string(), tok_type: caught_tok});
          }
        }
          
      }
    
      if !is_comment {
        break;
      }
    
    }
    
    return next_token;
  }
  
}