# Overview

This is a repo containing source code for a compiler written for a compilers class I took in the spring of 2019.
Given a source file written in a language specific to this class, the compiler will (eventually) output an executable program.

# Work done and work remaining
- [x] Lexer
- [x] Parser
- [x] Type-checker
- [ ] Code generation
- [ ] Runtime

# Setting up the project

## Installing rust
This project is written in rust and so to build it, you'll need to install some tooling.
[Here](https://www.rust-lang.org/tools/install) is an authoritative source on the instructions if more information is needed.

1. Install rustup (rust's installer tool) using the following command (and then following instructions to install from 'stable'): `curl https://sh.rustup.rs -sSf | sh`
2. Add to PATH env var by running the following/ adding it to your ~/.bashrc file: `export PATH="$HOME/.cargo/bin:${PATH}"`
3. Test installation by running: `rustc --version`

## Building and running
After you have cloned this repo, following the following steps to build and run the compiler:
1. Make sure you are in the directory of the project (the folder you cloned)
2. Build and run using `cargo run <filename>`, where filename is the path to the source file you're testing relative to your current directory.
  * Note: For my own development purposes, if no source file is selected `sample_programs/correct/source.src` is used by default.

## Running compiled code
After you have run the compiler, you will have a file named `<program_name>.bc` containing llvm bitcode. The following steps will obtain from this an executable file.
1. Run `llc -filetype=obj <program_name>.bc`. This will output an object file, `<program_name>.o`.
2. Run `gcc -o <desired_executable_name> <program_name>.o src/builtins/builtins.o` to link the program with builtins and output the executable.

## Building and Compiling *.src to executable with make
To build the compiler and run on a *.src file, run `make program NAME=<name>.src` to get an executable. The source file should be in `sample_programs/correct`.

## Uninstalling rust
If you'd like to remove rust from your machine, simply run: `rustup self uninstall`.


# About the project

## Structure of the project
The entry point for my compiler is `src/main.rs`. This entrypoint is responsible 
for opening the source file, obtaining a character stream and using this to start the Parser (`src/parser.rs`).
When the parser is started, it creates a new lexer (`src/lexer.rs`) using the provided program character stream.
The Lexer is a member of the Parser struct.

Other than these files, `src/tokenize` contains definitions for the Token enum 
which contains a state machine and is used by the lexer to build up lexemes from 
the given character stream. After writing behavior to distinguish between token 
types and to build Token enums accordingly, I found that I would need a struct 
to wrap around the Token enum so I could more easily operate on data that was 
generic to tokens. This is because rust's compiler is strict and ensures that 
behavior is defined for all enum variants at all times. Without a wrapper, I 
would need to repeat lots of code to define behavior for each and every token type.
After making the mistake of creating state machines for all of these token types, 
I created the `TokenEntry` object which would store common data like the line number,
type, and characters associated with a given lexeme (perhaps this would have been a better name).
By putting all of this data in a general struct rather than associating it with specific enum variants, 
I was avoiding the risk of the rust compiler thinking that operations on this 
data were token-type-specific and so requiring of token-type-specific definitions.


## Lexing

A very time-consuming part of this project was implementing
the state machines for each token type. A file for each of these definitions can be 
found in `src/tokens/*.rs`. These state machines use rust's pattern matching in 
a similar fashion to a switch statement. Each time the next state is requested 
along with a character to advance with, the state machine looks up valid 
transitions by comparing the tuple `(<curr_state>, <next_char>)`. If a valid 
transition is matched, the state is advanced accordingly and otherwise, the 
state is set to `Option::None`, indicating an invalid state.

The lexer distinguishes between tokens by starting with a vector of all token 
types and advancing the corresponding state machines while there is more 
than one valid token left. It was important to realize the distinction 
between accepted and not-invalid states so the lexer correctly handles the case 
where only 1 valid token is left in the vector, but it is not in an accept 
state.

Starting in the lexer I decided to handle the character stream as an iterator. 
A challenge that arose from this was knowing when to consume values from the iterator.
Originally I was advancing when advancing the state machines, but this was an 
issue since the state machines were advanced to an invalid state, which meant that 
the first character of the proceeding token would always be consumed too early.
Fortunately, to accompany the `Iterator` struct, rust provides a `Peekable` struct,
which provides a 1-lookahead. I was able to use this to advance all state 
machines and then decide to advance the character stream based on the result.

## Parsing

After using the `Iterator` and `Peekable` structs provided by rust, I decided 
refactor my lexer to conform to the corresponding interfaces. This allowed me to 
seamlessly obtain from my lexer a stream of tokens with a 1-lookahead.

The parser starts with one function per grammar rule. Additional parse rules 
were created for procedure_call&name, and name&number (within the 'factor' rule) 
since these had an ambiguity that required more than 1 lookahead. Additionally, 
grammar rules that needed to be left factors included an inner function in the parse function.
By doing this, the function could be called normally, but then use the refactored definition 
behind the scenes.

To handle errors, I created a ParserResult struct that is returned from each 
parse function. The result can be successful, or a specific type of error. 
Additionally, the result can be ignored or used by the caller as needed. This 
important when defining resyncing which occurs on statements and definitions 
with semicolon (;) as the resync point.

## Type checking

To begin type checking, I started by capturing the target types and passing 
these down throughout parse functions. Next I started by making types available 
from the factor parse rules. As these values are combined within an expression, 
the types are checked for compatibility with each other and with the type of 
combination being applied (i.e. comparison, addition, division, etc.). If an 
incompatible type is found, then an error ParserResult struct is passed through 
to the top to be printed at the statement  level. At the statement level, 
it is asserted that the expression's resolved type matches the type expected by the statement.

## Code generation

For code generation I used the [llvm_sys](http://rustdoc.taricorp.net/llvm-sys/llvm_sys/) rust crate (library).
This wraps around the llvm c api. Because Rust does provide raw pointer types for this kind of integration, 
my compiler is able to make calls into the llvm api and get back various llvm objects. 
At first in this process I found confusing about this process is when to use rust-style references and c-style 
raw pointers which are both present in Rust. On one hand, using Rust references allows values to be 
considered by the borrow checker leading to improved safety, but raw references are required to conform to llvm_sys'
apis.