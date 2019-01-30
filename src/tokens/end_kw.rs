// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct EndKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for EndKW {
  
  fn start() -> EndKW {
    return EndKW {state: Some(tokenize::State::new(0))};
  }
  
  fn next(&mut self, ch: char) {
    match &self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'e') => self.state = Some(tokenize::State::new(1)),
          (1, 'n') => self.state = Some(tokenize::State::new(2)),
          (2, 'd') => self.state = Some(tokenize::State::new(3).as_accept()),
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}