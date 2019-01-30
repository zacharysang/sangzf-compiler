// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct Period {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for Period {
  
  fn start() -> Period {
    return Period {state: Some(tokenize::State::new(0))};
  }
  
  fn next(&mut self, ch: char) {
    match &self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '.') => self.state = Some(tokenize::State::new(1).as_accept()),
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}