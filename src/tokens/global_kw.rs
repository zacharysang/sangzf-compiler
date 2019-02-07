// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct GlobalKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for GlobalKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::GlobalKW( GlobalKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'g') => {state_val.to(1, ch);},
          (1, 'l') => {state_val.to(2, ch);},
          (2, 'o') => {state_val.to(3, ch);},
          (3, 'b') => {state_val.to(4, ch);},
          (4, 'a') => {state_val.to(5, ch);},
          (5, 'l') => {state_val.to(6, ch).as_accept();},
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