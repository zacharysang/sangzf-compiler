use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;
use crate::tokenize::token::Token;
use crate::tokenize::char_group::CharGroup;


pub struct Identifier {
  pub state: Option<State>
}

impl Lexable for Identifier {
  
  fn start() -> Token {
    return Token::Identifier( Identifier {state: Some(State::new(0))} );
  }
  
  fn next(&mut self, ch: char) {
    match &mut self.state {
      Some(state_val) => {
        match (state_val.label, CharGroup::get(ch)) {
          (0, CharGroup::AlphaLower(ch)) => { state_val.to(1, ch).as_accept(); },
          (0, CharGroup::AlphaUpper(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, CharGroup::AlphaLower(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, CharGroup::AlphaUpper(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, CharGroup::Number(ch)) => { state_val.to(1, ch).as_accept(); },
          (1, CharGroup::Other('_')) => { state_val.to(1, ch).as_accept(); },
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