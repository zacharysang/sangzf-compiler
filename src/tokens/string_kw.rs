use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct StringKW {
  pub state: Option<State>
}

impl Lexable for StringKW {
  
  fn start() -> Token {
    return Token::StringKW( StringKW {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 's') => {state_val.to(1, ch);},
          (1, 't') => {state_val.to(2, ch);},
          (2, 'r') => {state_val.to(3, ch);},
          (3, 'i') => {state_val.to(4, ch);},
          (4, 'n') => {state_val.to(5, ch);},
          (5, 'g') => {state_val.to(6, ch).as_accept();}
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