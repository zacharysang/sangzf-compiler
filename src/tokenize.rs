use crate::tokens;

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
    self.chars.push(ch);
    
    return self;
  }
  
}

pub trait Lexable {

  // meant to get a start state for this token object
  fn start() -> Token;

  // returns true if the transition exists
  fn next(&mut self, ch: char);
  
}

pub enum Token {
  ProgramKW(tokens::program_kw::ProgramKW),
  BeginKW(tokens::begin_kw::BeginKW),
  EndKW(tokens::end_kw::EndKW),
  Period(tokens::period::Period),
  Semicolon(tokens::semicolon::Semicolon),
  IsKW(tokens::is_kw::IsKW),
  GlobalKW(tokens::global_kw::GlobalKW)
}


impl Token {

  pub fn next(&mut self, ch: char) -> &Option<State> {
  
    match self {
      Token::ProgramKW(tok) => { tok.next(ch); return &tok.state; },
      Token::BeginKW(tok) => { tok.next(ch); return &tok.state; },
      Token::EndKW(tok) => { tok.next(ch); return &tok.state; },
      Token::Period(tok) => { tok.next(ch); return &tok.state; },
      Token::Semicolon(tok) => { tok.next(ch); return &tok.state; },
      Token::IsKW(tok) => { tok.next(ch); return &tok.state; },
      Token::GlobalKW(tok) => { tok.next(ch); return &tok.state; }
    }
  
  }
  
  // get state
  pub fn get_state(&self) -> &Option<State> {
    match self {
      Token::ProgramKW(tok) => &tok.state,
      Token::BeginKW(tok) => &tok.state,
      Token::EndKW(tok) => &tok.state,
      Token::Period(tok) => &tok.state,
      Token::Semicolon(tok) => &tok.state,
      Token::IsKW(tok) => &tok.state,
      Token::GlobalKW(tok) => &tok.state
    }
  }
}

pub fn is_ws(ch: char) -> bool {
  return ch == ' ' || ch == '\t' || ch == '\n';
}

