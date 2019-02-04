// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct Number {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for Number {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::Number( Number {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}