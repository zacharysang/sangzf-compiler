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
    self.accept = false;
    self.chars.push(ch);
    
    return self;
  }
  
}

pub trait Lexable {

  // meant to get a start state for this token object
  fn start() -> Token;

  // returns true if the transition exists
  fn next(&mut self, ch: char);
  
  fn get_state(&self) -> &Option<State>;
  
}

pub struct TokenEntry {
  pub chars: String,
  pub tok_type: Token
}

pub enum Token {
  ProgramKW(tokens::program_kw::ProgramKW),
  BeginKW(tokens::begin_kw::BeginKW),
  EndKW(tokens::end_kw::EndKW),
  Period(tokens::period::Period),
  Semicolon(tokens::semicolon::Semicolon),
  IsKW(tokens::is_kw::IsKW),
  GlobalKW(tokens::global_kw::GlobalKW),
  ProcedureKW(tokens::procedure_kw::ProcedureKW),
  LParen(tokens::parens::LParen),
  RParen(tokens::parens::RParen),
  Comma(tokens::comma::Comma),
  VariableKW(tokens::variable_kw::VariableKW),
  TypeKW(tokens::type_kw::TypeKW),
  LBrace(tokens::braces::LBrace),
  RBrace(tokens::braces::RBrace),
  IntegerKW(tokens::integer_kw::IntegerKW),
  FloatKW(tokens::float_kw::FloatKW),
  StringKW(tokens::string_kw::StringKW),
  BoolKW(tokens::bool_kw::BoolKW),
  EnumKW(tokens::enum_kw::EnumKW),
  Dash(tokens::dash::Dash),
  IfKW(tokens::if_kw::IfKW),
  ThenKW(tokens::then_kw::ThenKW),
  ElseKW(tokens::else_kw::ElseKW),
  ForKW(tokens::for_kw::ForKW),
  ReturnKW(tokens::return_kw::ReturnKW),
  NotKW(tokens::not_kw::NotKW),
  Ampersand(tokens::ampersand::Ampersand),
  Plus(tokens::plus::Plus),
  TrueKW(tokens::true_kw::TrueKW),
  FalseKW(tokens::false_kw::FalseKW),
  LT(tokens::lt::LT),
  LTE(tokens::lte::LTE),
  GT(tokens::gt::GT),
  GTE(tokens::gte::GTE),
  EQ(tokens::eq::EQ),
  NEQ(tokens::neq::NEQ),
  Asterisk(tokens::asterisk::Asterisk),
  Slash(tokens::slash::Slash),
  Identifier(tokens::identifier::Identifier),
  Number(tokens::number::Number),
  String(tokens::string::String),
  LBracket(tokens::brackets::LBracket),
  RBracket(tokens::brackets::RBracket),
  Pipe(tokens::pipe::Pipe),
  Assign(tokens::assign::Assign),
  LineComment(tokens::line_comment::LineComment),
  MultilineComment(tokens::multiline_comment::MultilineComment),
  Unknown(tokens::unknown::Unknown)
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
      Token::GlobalKW(tok) => { tok.next(ch); return &tok.state; },
      Token::ProcedureKW(tok) => { tok.next(ch); return &tok.state; },
      Token::LParen(tok) => { tok.next(ch); return &tok.state; },
      Token::RParen(tok) => { tok.next(ch); return &tok.state; },
      Token::Comma(tok) => { tok.next(ch); return &tok.state; },
      Token::VariableKW(tok) => { tok.next(ch); return &tok.state; },
      Token::TypeKW(tok) => { tok.next(ch); return &tok.state; },
      Token::LBrace(tok) => { tok.next(ch); return &tok.state; },
      Token::RBrace(tok) => { tok.next(ch); return &tok.state; },
      Token::IntegerKW(tok) => { tok.next(ch); return &tok.state; },
      Token::FloatKW(tok) => { tok.next(ch); return &tok.state; },
      Token::StringKW(tok) => { tok.next(ch); return &tok.state; },
      Token::BoolKW(tok) => { tok.next(ch); return &tok.state; },
      Token::EnumKW(tok) => { tok.next(ch); return &tok.state; },
      Token::Dash(tok) => { tok.next(ch); return &tok.state; },
      Token::IfKW(tok) => { tok.next(ch); return &tok.state; },
      Token::ThenKW(tok) => { tok.next(ch); return &tok.state; },
      Token::ElseKW(tok) => { tok.next(ch); return &tok.state; },
      Token::ForKW(tok) => { tok.next(ch); return &tok.state; },
      Token::ReturnKW(tok) => { tok.next(ch); return &tok.state; },
      Token::NotKW(tok) => { tok.next(ch); return &tok.state; },
      Token::Ampersand(tok) => { tok.next(ch); return &tok.state; },
      Token::Plus(tok) => { tok.next(ch); return &tok.state; },
      Token::TrueKW(tok) => { tok.next(ch); return &tok.state; },
      Token::FalseKW(tok) => { tok.next(ch); return &tok.state; },
      Token::LT(tok) => { tok.next(ch); return &tok.state; },
      Token::GT(tok) => { tok.next(ch); return &tok.state; },
      Token::LTE(tok) => { tok.next(ch); return &tok.state; },
      Token::GTE(tok) => { tok.next(ch); return &tok.state; },
      Token::EQ(tok) => { tok.next(ch); return &tok.state; },
      Token::NEQ(tok) => { tok.next(ch); return &tok.state; },
      Token::Asterisk(tok) => { tok.next(ch); return &tok.state; },
      Token::Slash(tok) => { tok.next(ch); return &tok.state; },
      Token::Identifier(tok) => { tok.next(ch); return &tok.state; },
      Token::Number(tok) => { tok.next(ch); return &tok.state; },
      Token::String(tok) => { tok.next(ch); return &tok.state; },
      Token::LBracket(tok) => { tok.next(ch); return &tok.state; },
      Token::RBracket(tok) => { tok.next(ch); return &tok.state; },
      Token::Pipe(tok) => { tok.next(ch); return &tok.state; },
      Token::Assign(tok) => { tok.next(ch); return &tok.state; },
      Token::LineComment(tok) => { tok.next(ch); return &tok.state; },
      Token::MultilineComment(tok) => { tok.next(ch); return &tok.state; },
      Token::Unknown(tok) => { tok.next(ch); return &tok.state; }
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
      Token::GlobalKW(tok) => &tok.state,
      Token::ProcedureKW(tok) => &tok.state,
      Token::LParen(tok) => &tok.state,
      Token::RParen(tok) => &tok.state,
      Token::Comma(tok) => &tok.state,
      Token::VariableKW(tok) => &tok.state,
      Token::TypeKW(tok) => &tok.state,
      Token::LBrace(tok) => &tok.state,
      Token::RBrace(tok) => &tok.state,
      Token::IntegerKW(tok) => &tok.state,
      Token::FloatKW(tok) => &tok.state,
      Token::StringKW(tok) => &tok.state,
      Token::BoolKW(tok) => &tok.state,
      Token::EnumKW(tok) => &tok.state,
      Token::Dash(tok) => &tok.state,
      Token::IfKW(tok) => &tok.state,
      Token::ThenKW(tok) => &tok.state,
      Token::ElseKW(tok) => &tok.state,
      Token::ForKW(tok) => &tok.state,
      Token::ReturnKW(tok) => &tok.state,
      Token::NotKW(tok) => &tok.state,
      Token::Ampersand(tok) => &tok.state,
      Token::Plus(tok) => &tok.state,
      Token::TrueKW(tok) => &tok.state,
      Token::FalseKW(tok) => &tok.state,
      Token::LT(tok) => &tok.state,
      Token::GT(tok) => &tok.state,
      Token::LTE(tok) => &tok.state,
      Token::GTE(tok) => &tok.state,
      Token::EQ(tok) => &tok.state,
      Token::NEQ(tok) => &tok.state,
      Token::Asterisk(tok) => &tok.state,
      Token::Slash(tok) => &tok.state,
      Token::Identifier(tok) => &tok.state,
      Token::Number(tok) => &tok.state,
      Token::String(tok) => &tok.state,
      Token::LBracket(tok) => &tok.state,
      Token::RBracket(tok) => &tok.state,
      Token::Pipe(tok) => &tok.state,
      Token::Assign(tok) => &tok.state,
      Token::LineComment(tok) => &tok.state,
      Token::MultilineComment(tok) => &tok.state,
      Token::Unknown(tok) => &tok.state
    }
  }
  
}

pub fn is_ws(ch: char) -> bool {
  return ch == ' ' || ch == '\t' || ch == '\n';
}

pub enum CharGroup {
  AlphaLower(char),
  AlphaUpper(char),
  Number(char),
  Other(char)
}


impl CharGroup {
  pub fn get(ch: char) -> CharGroup {
    match ch {
      'a' => CharGroup::AlphaLower('a'),
      'b' => CharGroup::AlphaLower('b'),
      'c' => CharGroup::AlphaLower('c'),
      'd' => CharGroup::AlphaLower('d'),
      'e' => CharGroup::AlphaLower('e'),
      'f' => CharGroup::AlphaLower('f'),
      'g' => CharGroup::AlphaLower('g'),
      'h' => CharGroup::AlphaLower('h'),
      'i' => CharGroup::AlphaLower('i'),
      'j' => CharGroup::AlphaLower('j'),
      'k' => CharGroup::AlphaLower('k'),
      'l' => CharGroup::AlphaLower('l'),
      'm' => CharGroup::AlphaLower('m'),
      'n' => CharGroup::AlphaLower('n'),
      'o' => CharGroup::AlphaLower('o'),
      'p' => CharGroup::AlphaLower('p'),
      'q' => CharGroup::AlphaLower('q'),
      'r' => CharGroup::AlphaLower('r'),
      's' => CharGroup::AlphaLower('s'),
      't' => CharGroup::AlphaLower('t'),
      'u' => CharGroup::AlphaLower('u'),
      'v' => CharGroup::AlphaLower('v'),
      'w' => CharGroup::AlphaLower('w'),
      'x' => CharGroup::AlphaLower('x'),
      'y' => CharGroup::AlphaLower('y'),
      'z' => CharGroup::AlphaLower('z'),
      'A' => CharGroup::AlphaUpper('A'),
      'B' => CharGroup::AlphaUpper('B'),
      'C' => CharGroup::AlphaUpper('C'),
      'D' => CharGroup::AlphaUpper('D'),
      'E' => CharGroup::AlphaUpper('E'),
      'F' => CharGroup::AlphaUpper('F'),
      'G' => CharGroup::AlphaUpper('G'),
      'H' => CharGroup::AlphaUpper('H'),
      'I' => CharGroup::AlphaUpper('I'),
      'J' => CharGroup::AlphaUpper('J'),
      'K' => CharGroup::AlphaUpper('K'),
      'L' => CharGroup::AlphaUpper('L'),
      'M' => CharGroup::AlphaUpper('M'),
      'N' => CharGroup::AlphaUpper('N'),
      'O' => CharGroup::AlphaUpper('O'),
      'P' => CharGroup::AlphaUpper('P'),
      'Q' => CharGroup::AlphaUpper('Q'),
      'R' => CharGroup::AlphaUpper('R'),
      'S' => CharGroup::AlphaUpper('S'),
      'T' => CharGroup::AlphaUpper('T'),
      'U' => CharGroup::AlphaUpper('U'),
      'V' => CharGroup::AlphaUpper('V'),
      'W' => CharGroup::AlphaUpper('W'),
      'X' => CharGroup::AlphaUpper('X'),
      'Y' => CharGroup::AlphaUpper('Y'),
      'Z' => CharGroup::AlphaUpper('Z'),
      '0' => CharGroup::Number('0'),
      '1' => CharGroup::Number('1'),
      '2' => CharGroup::Number('2'),
      '3' => CharGroup::Number('3'),
      '4' => CharGroup::Number('4'),
      '5' => CharGroup::Number('5'),
      '6' => CharGroup::Number('6'),
      '7' => CharGroup::Number('7'),
      '8' => CharGroup::Number('8'),
      '9' => CharGroup::Number('9'),
      _ => CharGroup::Other(ch)
      
    }
  }
}

