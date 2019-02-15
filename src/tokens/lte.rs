use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct LTE {
  pub state: Option<State>
}

impl Lexable for LTE {
  
  fn start() -> Token {
    return Token::LTE( LTE {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '<') => {state_val.to(1, ch);},
          (1, '=') => {state_val.to(2, ch).as_accept();}
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