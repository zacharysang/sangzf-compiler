use crate::tokenize::state::State;
use crate::tokenize::lexable::Lexable;

use crate::tokens;

// import llvm dependencies
use llvm_sys::prelude::*;
use llvm_sys::{core};

pub struct TokenEntry {
  pub chars: String,
  pub tok_type: Token,
  pub line_num: u32,
  pub r#type: Type,
  pub value_ref: LLVMValueRef
}

impl TokenEntry {
  pub fn none_tok() -> TokenEntry {
    return TokenEntry {
      chars: String::from(""),
      tok_type: Token::Unknown(tokens::unknown::Unknown{state: None}),
      line_num: 0,
      r#type: Type::None,
      value_ref: unsafe { core::LLVMConstInt(core::LLVMInt32Type(), 0, 0) }
    }
  }
}

pub enum Token {
  ProgramKW(tokens::program_kw::ProgramKW),
  BeginKW(tokens::begin_kw::BeginKW),
  EndKW(tokens::end_kw::EndKW),
  Period(tokens::period::Period),
  Semicolon(tokens::semicolon::Semicolon),
  Colon(tokens::colon::Colon),
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
      Token::Colon(tok) => { tok.next(ch); return &tok.state; },
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
      Token::Colon(tok) => &tok.state,
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
  
  pub fn get_example(&self) -> &'static str {
    match self {
      Token::ProgramKW(_) => "program",
      Token::BeginKW(_) => "begin",
      Token::EndKW(_) => "end",
      Token::Period(_) => ".",
      Token::Semicolon(_) => ";",
      Token::Colon(_) => ":",
      Token::IsKW(_) => "is",
      Token::GlobalKW(_) => "global",
      Token::ProcedureKW(_) => "procedure",
      Token::LParen(_) => "(",
      Token::RParen(_) => ")",
      Token::Comma(_) => ",",
      Token::VariableKW(_) => "variable",
      Token::TypeKW(_) => "type",
      Token::LBrace(_) => "{",
      Token::RBrace(_) => "}",
      Token::IntegerKW(_) => "integer",
      Token::FloatKW(_) => "float",
      Token::StringKW(_) => "string",
      Token::BoolKW(_) => "bool",
      Token::EnumKW(_) => "enum",
      Token::Dash(_) => "-",
      Token::IfKW(_) => "if",
      Token::ThenKW(_) => "then",
      Token::ElseKW(_) => "else",
      Token::ForKW(_) => "for",
      Token::ReturnKW(_) => "return",
      Token::NotKW(_) => "not",
      Token::Ampersand(_) => "&",
      Token::Plus(_) => "+",
      Token::TrueKW(_) => "true",
      Token::FalseKW(_) => "false",
      Token::LT(_) => "<",
      Token::GT(_) => ">",
      Token::LTE(_) => "<=",
      Token::GTE(_) => ">=",
      Token::EQ(_) => "==",
      Token::NEQ(_) => "!=",
      Token::Asterisk(_) => "*",
      Token::Slash(_) => "/",
      Token::Identifier(_) => "<identifier>",
      Token::Number(_) => "<number>",
      Token::String(_) => "<string>",
      Token::LBracket(_) => "[",
      Token::RBracket(_) => "]",
      Token::Pipe(_) => "|",
      Token::Assign(_) => ":=",
      Token::LineComment(_) => "<line_comment>",
      Token::MultilineComment(_) => "<multiline_comment>",
      Token::Unknown(_) => "<other>"
    }
  }
}

#[derive(Clone)]
pub enum Type {
  None,
  Procedure(Vec<Box<Type>>,Box<Type>),
  Type(Box<Type>), // TODO add support for resolving type
  Enum,
  Integer,
  Float,
  String,
  Bool,
  Array(Box<Type>, u32),
  Custom(String)
}

impl ToString for Type {
  fn to_string(&self) -> String {
    return match self {
      Type::None => String::from(String::from("_")),
      Type::Procedure(params, result) => {
        
        let mut params_str = String::new();
        for param in params {
          params_str.push_str(&(*param.to_string()));
          params_str.push_str(",");
        }
        
        let mut proc_str = String::new();
        
        proc_str.push_str("procedure(");
        proc_str.push_str(&params_str);
        proc_str.push_str(") -> ");
        proc_str.push_str(&result.to_string());
        
        String::from(proc_str)
        
      },
      Type::Type(resolve) => {
        let mut type_str = String::from("type(");
        type_str.push_str(&resolve.to_string());
        type_str.push_str(")");
        
        type_str
      },
      Type::Enum => String::from("enum"),
      Type::Integer => String::from("integer"),
      Type::Float => String::from("float"),
      Type::String => String::from("string"),
      Type::Bool => String::from("bool"),
      Type::Array(el_type, size) => {
        let size = if size > &0 {
          size.to_string()
        } else {
          String::from("_")
        };
        let mut array_str = String::from("array(");
        
        array_str.push_str(&el_type.to_string());
        array_str.push_str(")[");
        array_str.push_str(&size[..]);
        array_str.push_str("]");
        
        array_str
        
      },
      Type::Custom(_) => String::from("custom"),
    };
  }
}