use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;

// Code for the left/open paren
pub struct LBrace {
  pub state: Option<State>
}

impl Lexable for LBrace {
  
  fn start() -> Token {
    return Token::LBrace( LBrace {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '{') => {state_val.to(1, ch).as_accept();},
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
pub struct RBrace {
  pub state: Option<State>
}

impl Lexable for RBrace {
  
  fn start() -> Token {
    return Token::RBrace( RBrace {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '}') => {state_val.to(1, ch).as_accept();},
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