use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;


pub struct MultilineComment {
  pub state: Option<State>
}

impl Lexable for MultilineComment {
  
  fn start() -> Token {
    return Token::MultilineComment( MultilineComment {state: Some(State::new(0))} );
  }

  fn next(&mut self, ch: char) {
  
    match &mut self.state {
      Some(state_val) => {
      
        // states 2, and {x | (x - 1) % 4 == 0} represent open comment states
        match (state_val.label > 7, state_val.label, (state_val.label + 1) % 3, ch) {
          (false, 0, _, '/') => {state_val.to(1, ch);},
          (false, 1, _, '*') => {state_val.to(2, ch);},
          (false, 2, _, '/') => {state_val.to(5, ch);},
          (false, 2, _, '*') => {state_val.to(3, ch);},
          (false, 2, _, _) => {state_val.to(2, ch);},
          (false, 3, _, '*') => {state_val.to(3, ch);},
          (false, 3, _, '/') => {state_val.to(4, ch).as_accept();},
          (false, 3, _, _) => {state_val.to(2, ch);},
          (false, 5, _, '/') => {state_val.to(5, ch);},
          (false, 5, _, '*') => {state_val.to(6, ch);},
          (false, 5, _, _) => {state_val.to(2, ch);},
          (false, 6, _, '/') => {state_val.to(8, ch);},
          (false, 6, _, '*') => {state_val.to(7, ch);},
          (false, 6, _, _) => {state_val.to(6, ch);},
          (false, 7, _, '*') => {state_val.to(7, ch);},
          (false, 7, _, '/') => {state_val.to(2, ch);},
          (false, 7, _, _) => {state_val.to(6, ch);},
          (true, _, 1, '/') => {state_val.to(state_val.label + 2, ch);},
          (true, _, 1, '*') => {state_val.to(state_val.label + 1, ch);},
          (true, _, 1, _) => {state_val.to(state_val.label, ch);},
          (true, _, 0, '/') => {state_val.to(state_val.label, ch);},
          (true, _, 0, '*') => {state_val.to(state_val.label + 1, ch);},
          (true, _, 0, _) => {state_val.to(state_val.label - 2, ch);},
          (true, _, 2, '*') => {state_val.to(state_val.label, ch);},
          (true, _, 2, '/') => {state_val.to(state_val.label - 4, ch);},
          (true, _, 2, _) => {state_val.to(state_val.label - 2, ch);}
          _ => self.state = None
        }
        
        
      },
      None => ()
    }
  }
  
  fn get_state(&self) -> &Option<State> {
    return &self.state;
  }
  
}