#[macro_use]
extern crate nom;
mod builtins;
mod parsing;
mod evals;
use nom::types::CompleteStr as Input;
use std::io;
use std::io::Write;
use rlisp::*;
use crate::builtins::*;
use crate::parsing::*;
use crate::evals::*;
    
fn main() {
    let stdin = io::stdin();
    let mut env = build_init_env();

    loop {
        let mut buffer = String::new();
        print!("user>");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();
        if buffer.len() == 0 {
            break;
        }
        match rep(buffer, &mut env) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Runtime Error: {:?}", e),
        }
    }
    print!("\n");
    
}


fn build_init_env() -> Env {
    let mut env = Env::new();
    env.set(String::from("+"), Value::Function(plus));
    env.set(String::from("*"), Value::Function(mult));
    env
}

    

#[allow(non_snake_case)]
fn READ(input: String) -> Result<RValue, Errors>{
    match parse_value(Input(&input)) {
        Ok(v) => Ok(v.1),
        Err(_) => Err(Errors::ParseError),
    }
}

#[allow(non_snake_case)]
fn PRINT(input: RValue) -> String{
    input.to_string()
}

fn rep(input: String, env: &mut Env) -> Result<String, Errors>{
    let ast = READ(input)?;
    let result = EVAL(ast , env)?;
    Ok(PRINT(result))
}


