// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct BeginKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for BeginKW {
  
  fn start() -> BeginKW {
    return BeginKW {state: Some(tokenize::State::new(0))};
  }
  
  fn next(&mut self, ch: char) {
    match &self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'b') => self.state = Some(tokenize::State::new(1)),
          (1, 'e') => self.state = Some(tokenize::State::new(2)),
          (2, 'g') => self.state = Some(tokenize::State::new(3)),
          (3, 'i') => self.state = Some(tokenize::State::new(4)),
          (4, 'n') => self.state = Some(tokenize::State::new(5).as_accept()),
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}