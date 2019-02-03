// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct ForKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for ForKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::ForKW( ForKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'f') => {state_val.to(1, ch);},
          (1, 'o') => {state_val.to(2, ch);},
          (2, 'r') => {state_val.to(3, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}