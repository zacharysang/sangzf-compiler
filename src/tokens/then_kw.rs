use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct ThenKW {
  pub state: Option<State>
}

impl Lexable for ThenKW {
  
  fn start() -> Token {
    return Token::ThenKW( ThenKW {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 't') => {state_val.to(1, ch);},
          (1, 'h') => {state_val.to(2, ch);},
          (2, 'e') => {state_val.to(3, ch);},
          (3, 'n') => {state_val.to(4, ch).as_accept();}
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