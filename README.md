# RLisp

RLisp is a simple Lisp intepreter written in Rust. I'm writing it for the purpose of learning Rust.

## Working
1. Parsing, parsing is implemented using _Parsec_
2. Basic evaluation, Lists represent function calls, symbols perform lookups
3. Special Forms: 
   * `define`, binds a value to a name in scope
   * `let`, creates a new scope, then binds a value to a name
   * `if`, conditional
   * `fn` Create functions
   * `eval` Evaluate the given expression
   * `quote` Don't evaluate the given expression
   

