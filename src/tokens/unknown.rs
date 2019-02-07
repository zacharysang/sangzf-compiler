// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct Unknown {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for Unknown {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::Unknown( Unknown {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    if let Some(state_val) = &mut self.state {
      state_val.to(0, ch);
    }
  }
  
  fn get_state(&self) -> &Option<tokenize::State> {
    return &self.state;
  }
  
}