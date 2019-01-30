// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct IsKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for IsKW {
  
  fn start() -> IsKW {
    return IsKW {state: Some(tokenize::State::new(0))};
  }
  
  fn next(&mut self, ch: char) {
    match &self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'i') => self.state = Some(tokenize::State::new(1)),
          (1, 's') => self.state = Some(tokenize::State::new(2).as_accept()),
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}