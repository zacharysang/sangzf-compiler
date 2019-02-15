use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;

pub struct EndKW {
  pub state: Option<State>
}

impl Lexable for EndKW {
  
  fn start() -> Token {
    return Token::EndKW( EndKW {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'e') => {state_val.to(1, ch);},
          (1, 'n') => {state_val.to(2, ch);},
          (2, 'd') => {state_val.to(3, ch).as_accept();},
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