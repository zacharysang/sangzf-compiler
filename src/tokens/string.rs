// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct String {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for String {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::String( String {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, tokenize::CharGroup::get(ch)) {
          (0, tokenize::CharGroup::Other('"')) => { state_val.to(1, ch); },
          (1, tokenize::CharGroup::Other('"')) => { state_val.to(2, ch).as_accept(); },
          (1, _) => { state_val.to(1, ch); }
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}