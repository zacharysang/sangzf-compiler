// bring tokenize module into scope so we can use token-related utilities
use crate::tokenize;

pub struct ProgramKW {
  pub state: Option<tokenize::State>
}

impl tokenize::Lexable for ProgramKW {

  fn start() -> tokenize::Token {
    return tokenize::Token::ProgramKW(ProgramKW {state: Some(tokenize::State::new(0))});
  }

  fn next(&mut self, ch: char) {
  
    // we match a mutable reference to this enum so we only borrow the value
    match &mut self.state {
    
      Some(state_val) => {
        
        match (state_val.label, ch) {
          (0, 'p') => {state_val.to(1, ch);},
          (1, 'r') => {state_val.to(2, ch);},
          (2, 'o') => {state_val.to(3, ch);},
          (3, 'g') => {state_val.to(4, ch);},
          (4, 'r') => {state_val.to(5, ch);},
          (5, 'a') => {state_val.to(6, ch);}
          (6, 'm') => {state_val.to(7, ch).as_accept();},
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