// bring this into scope so that token-related utilities can be used
use crate::tokenize;

// Code for the left/open paren
pub struct LBrace {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for LBrace {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::LBrace( LBrace {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '{') => {state_val.to(1, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<tokenize::State> {
    return &self.state;
  }
  
}


// Code for the right/close paren
pub struct RBrace {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for RBrace {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::RBrace( RBrace {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, '}') => {state_val.to(1, ch).as_accept();},
          _ => self.state = None
        }
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<tokenize::State> {
    return &self.state;
  }
  
}