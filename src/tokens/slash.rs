// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct Slash {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for Slash {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::Slash( Slash {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '/') => {state_val.to(1, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}