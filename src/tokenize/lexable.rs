use crate::tokenize::state::State;
use crate::tokenize::token::Token;

pub trait Lexable {

  // meant to get a start state for this token object
  fn start() -> Token;

  // returns true if the transition exists
  fn next(&mut self, ch: char);
  
  fn get_state(&self) -> &Option<State>;
  
}