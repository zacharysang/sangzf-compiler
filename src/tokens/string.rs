use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct String {
  pub state: Option<State>
}

impl Lexable for String {
  
  fn start() -> Token {
    return Token::String( String {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '"') => { state_val.to(1, ch); },
          (1, '"') => { state_val.to(2, ch).as_accept(); },
          (1, _) => { state_val.to(1, ch); }
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