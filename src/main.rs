use std::collections::HashSet;
use std::clone::Clone;

// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// bring Token into scope so we can use its associated items
use crate::tokenize::Token;


fn main() {

  let program = String::from("program begin end program.");

  // make a collection with state for all token types
  
  
  /*
  tokenTypes.insert(Box::new(tokens::program_kw::start()));
  tokenTypes.insert(Box::new(tokens::begin_kw::start()));
  tokenTypes.insert(Box::new(tokens::end_kw::start()));
  tokenTypes.insert(Box::new(tokens::period::start()));
  tokenTypes.insert(Box::new(tokens::is_kw::start()));
  
  let mut currTokenTypes = tokenTypes.clone();
  
  let mut tokens : Vec<Box<Token>> = Vec::new();
  
  let symbols = program.chars();
  
  let mut curr = symbols.next();
  while let Some(ch) = curr {
    
    // advance all token types
    for token in currTokenTypes {
      token.next(ch);
      
      // remove if invalid state
      if token.state.is_none() {
        currTokenTypes.remove(token);
      }
      
    }
    
    // if there is only 1 token type, then add it to tokens list
    if currTokenTypes.len() == 1 {
      
      let lexemes : LinkedList<Box<Token>> = currTokenTypes.drain().collect();
      tokens.append(lexemes);
      
      currTokenTypes = tokenTypes.clone();
      
    } else if currTokenTypes.len() == 0 {
      // error - no eligible token type
      println!("Error! - no eligible token type");
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
  */
}