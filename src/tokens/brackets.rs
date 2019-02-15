use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


// Code for the left/open paren
pub struct LBracket {
  pub state: Option<State>
}

impl Lexable for LBracket {
  
  fn start() -> Token {
    return Token::LBracket( LBracket {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '[') => {state_val.to(1, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<State> {
    return &self.state;
  }
  
}


// Code for the right/close paren
pub struct RBracket {
  pub state: Option<State>
}

impl Lexable for RBracket {
  
  fn start() -> Token {
    return Token::RBracket( RBracket {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, ']') => {state_val.to(1, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<State> {
    return &self.state;
  }
  
}