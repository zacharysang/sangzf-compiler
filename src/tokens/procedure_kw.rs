// bring this into scope so that token-related utilities can be used
use crate::tokenize;

pub struct ProcedureKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for ProcedureKW {
  
  fn start() -> tokenize::Token {
    return tokenize::Token::ProcedureKW( ProcedureKW {state: Some(tokenize::State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, ch) {
          (0, 'p') => {state_val.to(1, ch);},
          (1, 'r') => {state_val.to(2, ch);},
          (2, 'o') => {state_val.to(3, ch);},
          (3, 'c') => {state_val.to(4, ch);},
          (4, 'e') => {state_val.to(5, ch);},
          (5, 'd') => {state_val.to(6, ch);},
          (6, 'u') => {state_val.to(7, ch);},
          (7, 'r') => {state_val.to(8, ch);},
          (8, 'e') => {state_val.to(9, ch).as_accept();}
          _ => self.state = None
        }
      },
      None => ()
    }
  }
}