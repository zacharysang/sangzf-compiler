// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct BoolKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for BoolKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::BoolKW( BoolKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'b') => {state_val.to(1, ch);},
          (1, 'o') => {state_val.to(2, ch);},
          (2, 'o') => {state_val.to(3, ch);},
          (3, 'l') => {state_val.to(4, ch).as_accept();}
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}