// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct BeginKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for BeginKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::BeginKW( BeginKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'b') => {state_val.to(1, ch);},
          (1, 'e') => {state_val.to(2, ch);},
          (2, 'g') => {state_val.to(3, ch);},
          (3, 'i') => {state_val.to(4, ch);},
          (4, 'n') => {state_val.to(5, ch).as_accept();}
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