pub struct State {
  pub label: u32,
  pub accept: bool,
  pub chars: String
}

impl State {

  // with default arg accept=false
  pub fn new(label: u32) -> State {
    return State {label: label, accept: false, chars: String::from("")};
  }
  
  pub fn as_accept(&mut self) -> &mut State {
    self.accept = true;
  
    return self;
  }
  
  pub fn to(&mut self, label: u32, ch: char) -> &mut State {
    self.label = label;
    self.accept = false;
    self.chars.push(ch);
    
    return self;
  }
  
}
