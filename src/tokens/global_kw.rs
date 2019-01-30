// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct GlobalKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for GlobalKW {
  
  fn start() -> GlobalKW {
    return GlobalKW {state: Some(tokenize::State::new(0))};
  }
  
  fn next(&mut self, ch: char) {
    match &self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'g') => self.state = Some(tokenize::State::new(1)),
          (1, 'l') => self.state = Some(tokenize::State::new(2)),
          (2, 'o') => self.state = Some(tokenize::State::new(3)),
          (3, 'b') => self.state = Some(tokenize::State::new(4)),
          (4, 'a') => self.state = Some(tokenize::State::new(5)),
          (5, 'l') => self.state = Some(tokenize::State::new(6).as_accept()),
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}