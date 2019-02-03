// bring this into scope so that token-related utilities can be used
use crate::tokenize;
pub struct VariableKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for VariableKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::VariableKW( VariableKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'v') => {state_val.to(1, ch);},
          (1, 'a') => {state_val.to(2, ch);},
          (2, 'r') => {state_val.to(3, ch);},
          (3, 'i') => {state_val.to(4, ch);},
          (4, 'a') => {state_val.to(5, ch);},
          (5, 'b') => {state_val.to(6, ch);},
          (6, 'l') => {state_val.to(7, ch);},
          (7, 'e') => {state_val.to(8, ch).as_accept();}
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}