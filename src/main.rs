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
  
    // on whitespace, attempt to pull out the token
    // there should be only one eligible, else there is an error
    if tokenize::is_ws(ch) {
    
      let mut num_eligible = 0;
      let mut eligible_idx = None;
      
      // take inventory of valid tokens at point of whitespace
      for (i, token) in token_types.iter().enumerate() {
        match token.get_state() {
          None => (),
          Some(state) => {
          
            if state.accept {
              num_eligible += 1;
              eligible_idx = Some(i);
            }
            
          }
        }
      }
      
      // if there is an eligible idx, see if it is unique
      if let Some(idx) = eligible_idx {
      
        if num_eligible == 1 {
          // emit this token
          let tok = &token_types[idx];
          let state = tok.get_state();
          if let Some(state_val) = state {
            println!("(at ws) got token: {}", state_val.chars);
          }
          
        } else {
          // error no unique token at whitespace
          println!("Error! - Reached whitespace without forming a unique token");
        }
        
      } else {
        // error no valid tokens at whitespace
        println!("Error! - Reached whitespace without forming a valid token");
      }
      
      
      // at a whitespace, always restart search for next token
      token_types = get_all_types();
      curr = symbols.next();
      continue;
    }
    
    // advance all token types
    let mut remaining = token_types.len();
    let mut lexeme = None;
    
    // cannot use '&mut' here since iter_mut is giving up ownership of the IterMut struct and we must take this ownership
    // 'token' does not need to be mutable since we are working with and IterMut object. This object owns a mutable reference to the value we are modifying
    for (i, token) in token_types.iter_mut().enumerate() {
    
      let state = token.next(ch);
      
      match state {
        Some(state_val) => {
          if state_val.accept {
            lexeme = Some(token);
          }
        },
        None => {remaining -= 1;}
      }
    }
    
    // if there is only 1 token type, then add it to tokens list
    if remaining == 1 {
      
      //let lexemes : LinkedList<Box<Token>> = currTokenTypes.drain().collect();
      //tokens.append(lexemes);
      
      // will put into symbol table here
      
      // refresh the list list of token types
      if let Some(tok) = lexeme {
        if let Some(state_val) = tok.get_state() {
          println!("**got token with chars: '{}' and state {}", state_val.chars, state_val.label);
          
          token_types = get_all_types();
        }
      }
      
    } else if remaining == 0 {
      // error - no eligible token type
      println!("Error! - no eligible token type with ch: '{}'", ch);
      break;
    }
    
    curr = symbols.next();
  }
  
  // after reading the whole program, check if there is a token left
  /*
  for token in tokens {
    println!("a token");
  }
  */
}