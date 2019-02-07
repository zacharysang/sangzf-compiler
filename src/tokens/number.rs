// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct Number {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for Number {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::Number( Number {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, tokenize::CharGroup::get(ch)) {
          (0, tokenize::CharGroup::Number(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, tokenize::CharGroup::Number(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, tokenize::CharGroup::Other('_')) => { state_val.to(1, ch).as_accept(); },
          (1, tokenize::CharGroup::Other('.')) => { state_val.to(2, ch).as_accept(); },
          (2, tokenize::CharGroup::Number(ch)) => { state_val.to(2, ch).as_accept(); },
          (2, tokenize::CharGroup::Other('_')) => { state_val.to(2, ch).as_accept(); }
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