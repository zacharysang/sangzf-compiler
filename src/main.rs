// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// bring Token into scope so we can use its associated items
use crate::tokenize::Token;
use crate::tokenize::Lexable;

fn get_all_types<'a>() -> Box<Vec<Token>> {
  let mut token_types : Box<Vec<Token>> = Box::new(Vec::new());
  
  token_types.push(tokens::program_kw::ProgramKW::start());
  token_types.push(tokens::begin_kw::BeginKW::start());
  token_types.push(tokens::end_kw::EndKW::start());
  token_types.push(tokens::period::Period::start());
  token_types.push(tokens::semicolon::Semicolon::start());
  token_types.push(tokens::is_kw::IsKW::start());
  token_types.push(tokens::global_kw::GlobalKW::start());
  
  return token_types;
}

fn main() {

  let program = String::from("global program begin is end program.");

  // make a collection with state for all token types
  let mut token_types = get_all_types();
  
  let mut lexemes : Vec<Box<Token>> = Vec::new();
  
  let mut symbols = program.chars();
  
  let mut curr = symbols.next();
  while let Some(ch) = curr {
  
    // on whitespace check that we aren't in the middle of a token's machine
    // instead we should be in state 0 for all types
    if tokenize::is_ws(ch) {
    
      // count the number of types that are in the middle of the state machine
      let mut num_middle = 0;
      
      // take inventory of valid tokens at point of whitespace
      for token in token_types.iter() {
        if let Some(state) = token.get_state() {
          if state.label != 0 { 
            num_middle += 1; 
            break;
          }
        }
      }
      
      // if there is an eligible idx, see if it is unique
      if num_middle != 0 {
        // error no unique token at whitespace
        println!("Error! - Found whitespace without forming a token");
      }
      
      // reset the list of token type candidates
      token_types = get_all_types();
      
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
            println!("**got token with chars: '{}' and state {}", state_val.chars, state_val.label);
            
            // reset the list of token type candidates
            token_types = get_all_types();
          }
        }
        
      } else if remaining == 0 {
        // error - no eligible token type
        println!("Error! - No eligible token type with ch: '{}'", ch);
      }
      
    }

    // get the next character in the program
    curr = symbols.next();
  }

}