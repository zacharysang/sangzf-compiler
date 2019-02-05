// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// bring Token into scope so we can use its associated items
use crate::tokenize::Token;
use crate::tokenize::Lexable;
use crate::tokenize::TokenEntry;

// TODO move a lot of this into a Lexer.rs
// TODO Lexer.rs should read in files
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
  
  token_types.push(tokens::identifier::Identifier::start());
  token_types.push(tokens::number::Number::start());
  token_types.push(tokens::string::String::start());
  
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

fn next_tok(program: &mut std::str::Chars) -> Option<TokenEntry> {
  
  // make a collection with state for all token types
  let mut token_types = all_tokens();
    
  // information about an acceptable token
  // (not accepted until last option)
  let mut acceptable_idx = None;
  let mut acceptable_chars = None;
  
  // the value we will eventually return
  let mut next_token = None;
  
  let mut curr_ch = program.next();
  while let Some(ch) = curr_ch {
    if !tokenize::is_ws(ch) {
      break;
    }
    
    curr_ch = program.next();
  }
  
  
  // while at least one token is alive, keep adding characters
  // if the token is acceptable, save it
  // when none are alive state, return accept token
  // if acceptable == None, error
  let mut alive = true;
  while alive {
  
    alive = false;
    
    if let Some(ch) = curr_ch {
    
      for (i, token) in token_types.iter_mut().enumerate() {
      
        // advance curr type
        if let Some(state) = token.next(ch) {
          alive = true;
          
          // check if acceptable
          if state.accept {
          
            // make a new Token that matches this element
            acceptable_idx = Some(i);
            
            // get a copy of the string
            acceptable_chars = Some(String::from(&state.chars[..]));
          }
        }
        
      }
    }
    
    // advance the character iterator unless dead (= about to exit loop)
    if alive {
      curr_ch = program.next();
    }
    
  }
  
  if let Some(idx) = acceptable_idx {
  
    // print acceptable chars
    if let Some(chars) = acceptable_chars {
      if let Some(tok_type) = token_types.drain(idx..idx+1).next() {
        next_token = Some(TokenEntry { chars: chars, tok_type: tok_type });
      }
    }

  } else {
    println!("Error! - Could not produce token. Returning None");
  }
  
    return next_token;
  
}

fn main() {

  let mut counter = 0;
  
  // test program
  let program = String::from("program procedure global variable begin end is type integer float string bool enum if then else for return not true false . ; ( ) , { } - & + < > <= >= == != * / [ ] | := abcdef 1234 898.99 \"stringgoeshere\"  \"fancie$t  string_g0es\n\nhere\t\"");
  
  let mut program_chars = program.chars();

  // TODO change this to read until EOF is reached (instead of until no valid token)
  let mut token = next_tok(&mut program_chars);
  
  while let Some(tok) = token {
  
    println!("got token with chars: {}", tok.chars);

    token = next_tok(&mut program_chars);
    counter += 1;
  }
    
  println!("num tokens: {}", counter);
  
}