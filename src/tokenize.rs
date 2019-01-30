pub struct State {
  pub label: u32,
  pub accept: bool
}

impl State {

  // with default arg accept=false
  pub fn new(label: u32) -> State {
    return State {label: label, accept: false};
  }
  
  pub fn as_accept(mut self) -> State {
    self.accept = true;
  
    return self;
  }
  
}

pub trait Token {

  // meant to get a start state for this token object
  fn start() -> Self;

  // returns true if the transition exists
  fn next(&mut self, ch: char);
  
}
