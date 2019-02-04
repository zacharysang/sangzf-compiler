// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// bring Token into scope so we can use its associated items
use crate::tokenize::Token;
use crate::tokenize::Lexable;

fn all_keywords() -> Box<Vec<Token>> {
  let mut keywords = Box::new(Vec::new());
  
  keywords.push(tokens::program_kw::ProgramKW::start());
  keywords.push(tokens::begin_kw::BeginKW::start());
  keywords.push(tokens::end_kw::EndKW::start());
  keywords.push(tokens::is_kw::IsKW::start());
  keywords.push(tokens::global_kw::GlobalKW::start());
  keywords.push(tokens::procedure_kw::ProcedureKW::start());
  keywords.push(tokens::variable_kw::VariableKW::start());
  keywords.push(tokens::type_kw::TypeKW::start());
  keywords.push(tokens::integer_kw::IntegerKW::start());
  keywords.push(tokens::float_kw::FloatKW::start());
  keywords.push(tokens::string_kw::StringKW::start());
  keywords.push(tokens::bool_kw::BoolKW::start());
  keywords.push(tokens::enum_kw::EnumKW::start());
  keywords.push(tokens::if_kw::IfKW::start());
  keywords.push(tokens::then_kw::ThenKW::start());
  keywords.push(tokens::else_kw::ElseKW::start());
  keywords.push(tokens::for_kw::ForKW::start());
  keywords.push(tokens::return_kw::ReturnKW::start());
  keywords.push(tokens::not_kw::NotKW::start());
  keywords.push(tokens::true_kw::TrueKW::start());
  keywords.push(tokens::false_kw::FalseKW::start());
  
  return keywords;
  
}

fn all_tokens() -> Box<Vec<Token>> {

  // TODO put all these into a direct declaration rather than adding them all in
  let mut token_types = Box::new(Vec::new());
  
  token_types.push(tokens::period::Period::start());
  token_types.push(tokens::semicolon::Semicolon::start());
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
  
  //token_types.push(tokens::identifier::Identifier::start());
  //token_types.push(tokens::number::Number::start());
  //token_types.push(tokens::string::String::start());
  
  // These must be last so that they overwrite others
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
  
  return token_types;
}

fn main() {

  let mut counter = 0;
  
  // test program
  let program = String::from("program procedure global variable begin end is type integer float string bool enum if then else for return not true false . ; ( ) , { } - & + < > <= >= == != * / [ ] | :=");
  
  let mut program_chars = program.chars();

  // make a collection with state for all token types
  let mut token_types = all_tokens();
  
  // iterate over all chars in the program
  let mut curr = program_chars.next();
  while let Some(ch) = curr {
  
    // on whitespace check that we aren't in the middle of a token's machine
    // instead we should be in state 0 for all types
    if tokenize::is_ws(ch) {
    
      // count the number of types that are in the middle of the state machine
      let mut num_middle = 0;
      let mut num_accept = 0;
      let mut lexeme = None;
      
      // take inventory of valid tokens at point of whitespace
      for token in token_types.iter() {
      
        if let Some(state) = token.get_state() {
          if state.accept {
            num_accept += 1;
            lexeme = Some(token);
          } else if state.label != 0 { 
            num_middle += 1; 
          }
        }
      }
      
      if num_accept == 1 {
        if let Some(tok) = lexeme {
        
          if let Some(state_val) = tok.get_state() {
            println!("**got token (@ws) with chars: '{}' and state {}", state_val.chars, state_val.label);
            counter += 1;
          }
          
        }
      } else if num_middle != 0 {
        // error no unique token at whitespace
        println!("Error! - Found whitespace without forming a token");
      }
      
      // reset the list of token type candidates
      token_types = all_tokens();
      
      
    } else {
    
    
      // advance all token types until there is only 1 that isn't 'None'
      let mut remaining = token_types.len();
      let mut lexeme = None;
  
      // cannot use '&mut' here since iter_mut is giving up ownership of the IterMut struct and we must take this ownership
      // 'token' does not need to be mutable since we are working with and IterMut object. This object owns a mutable reference to the value we are modifying
      for token in token_types.iter_mut() {
      
        match token.next(ch) {
          Some(state_val) => {
            if state_val.accept {
              lexeme = Some(token);
            }
          },
          None => {
            remaining -= 1;
          }
        }
      }
      
      
      // emit token if there is only 1 token type (all others are 'None')
      if remaining == 1 {
        
        // will put into symbol table here
        
        // reset the list of token types
        if let Some(tok) = lexeme {
          if let Some(state_val) = tok.get_state() {
            counter += 1;
            println!("**got token with chars: '{}' and state {}", state_val.chars, state_val.label);
            
            // reset the list of token type candidates
            token_types = all_tokens();
          }
        }
        
      } else if remaining == 0 {
        // error - no eligible token type
        println!("Error! - No eligible token type with ch: '{}'", ch);
        
        // restart search
        token_types = all_tokens();
      } else if let Some(tok) = lexeme {
      
        // if remaining > 1, check if lexeme is a kw
        
        if tok.is_kw() {
          if let Some(state_val) = tok.get_state() {
            counter += 1;
            println!("** got token (keyword): '{}'", state_val.chars);
            
            // reset the list of token type candidates
            token_types = all_tokens();
          }
        }
        
      }
      
    }

    // get the next character in the program
    curr = program_chars.next();
  }
  
  println!("num tokens: {}", counter);

}