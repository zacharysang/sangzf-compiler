// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct NotKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for NotKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::NotKW( NotKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'n') => {state_val.to(1, ch);},
          (1, 'o') => {state_val.to(2, ch);},
          (2, 't') => {state_val.to(3, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}