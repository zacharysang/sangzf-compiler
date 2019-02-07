// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct TrueKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for TrueKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::TrueKW( TrueKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 't') => {state_val.to(1, ch);},
          (1, 'r') => {state_val.to(2, ch);},
          (2, 'u') => {state_val.to(3, ch);},
          (3, 'e') => {state_val.to(4, ch).as_accept();}
          _ => self.state = None
        }
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<tokenize::State> {
    return &self.state;
  }
  
}