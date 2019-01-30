// bring tokenize module into scope so we can use token-related utilities
use crate::tokenize;

pub struct ProgramKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Token for ProgramKW {

  fn start() -> ProgramKW {
    return ProgramKW {state: Some(tokenize::State::new(0))};
  }

  fn next(&mut self, ch: char) {

    // we match a mutable reference to this enum so we only borrow the value
    match &self.state {
    
      Some(state_val) => {
        
        match (state_val.label, ch) {
          (0, 'p') => self.state = Some(tokenize::State::new(1)),
          (1, 'r') => self.state = Some(tokenize::State::new(2)),
          (2, 'o') => self.state = Some(tokenize::State::new(3)),
          (3, 'g') => self.state = Some(tokenize::State::new(4)),
          (4, 'r') => self.state = Some(tokenize::State::new(5)),
          (5, 'a') => self.state = Some(tokenize::State::new(6)),
          (6, 'm') => self.state = Some(tokenize::State::new(7).as_accept()),
          _ => self.state = None
        }
        
      },
      None => ()
    }
  }
}