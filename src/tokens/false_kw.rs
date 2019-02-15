use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct FalseKW {
  pub state: Option<State>
}

impl Lexable for FalseKW {
  
  fn start() -> Token {
    return Token::FalseKW( FalseKW {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'f') => {state_val.to(1, ch);},
          (1, 'a') => {state_val.to(2, ch);},
          (2, 'l') => {state_val.to(3, ch);},
          (3, 's') => {state_val.to(4, ch);},
          (4, 'e') => {state_val.to(5, ch).as_accept();}
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