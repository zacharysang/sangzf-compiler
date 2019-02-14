// expose token utilities (State struct, Token trait)
mod tokenize;

// expose tokens
mod tokens;

// expose lexer
mod lexer;

fn main() {

  let mut lexer = lexer::Lexer {};

  let mut counter = 0;
  
  // test program
  
  // should have 51 tokens
  let program = String::from(" program procedure global   ??? variable begin end is type integer float string bool enum if then else for return not true false . ; ( ) , { } - & + < > <= >= == != * / [ ] | := abcdef 1234 898.99 \"stringgoeshere\" \"fancie$t  string_g0es\n\nhere\t\"");
  
  // should have 3 tokens
  //let program = String::from("abc/**this is /*///*a***/*/ doc*/+bcd");
  
  // should have 44 tokens
  let program = String::from("Program MyTestProgram is

integer i;
char c;
integer myArray[7:15];

begin

i := 100;
c := \"A\";

if(i > 100) then
//do something#
	i:= 1110;
end if;

/*Single #Line Block Comments*/

/*
/*
Multiline Block Comments*/
*/

end program.
");
  
  let mut program_chars = program.chars().peekable();

  while let Some(tok) = lexer.next_tok(&mut program_chars) {
  
    println!("got token with chars: '{}' ({})", tok.chars, counter);

    counter += 1;
  }
    
  println!("num tokens: {}", counter);
  
}