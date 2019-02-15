use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct Unknown {
  pub state: Option<State>
}

impl Lexable for Unknown {
  
  fn start() -> Token {
    return Token::Unknown( Unknown {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    if let Some(state_val) = &mut self.state {
      state_val.to(0, ch);
    }
  }
  
  fn get_state(&self) -> &Option<State> {
    return &self.state;
  }
  
}