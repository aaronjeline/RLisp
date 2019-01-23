#[macro_use]
extern crate nom;
mod builtins;
mod parsing;
mod evals;
mod stdlib;
use nom::types::CompleteStr as Input;
use std::io;
use std::io::Write;
use std::process;
use rlisp::*;
use crate::builtins::build_init_env;
use crate::parsing::*;
use crate::evals::*;
use crate::stdlib::*;
    
fn main() {
    let stdin = io::stdin();
    let mut env = build_init_env();
    match run_std_lib(&mut env) {
        Ok(_) => (),
        Err(e) => {
            println!("Error loading standard library!\n{:?}", e);
            process::exit(1);
        }
    }

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


fn run_std_lib(env: &mut Env) -> Result<(),Errors> {
    let fs = get_std_lib();
    for f in fs {
        rep(f, env)?;
    }
    Ok(())
}
    

#[allow(non_snake_case)]
fn READ(input: String) -> Result<RValue, Errors>{
    match parse_value(Input(&input)) {
        Ok(v) => Ok(v.1),
        Err(pe) => {
            let msg = pe.to_string();
            Err(Errors::ParseError(msg))
        },
    }
}


fn rep(input: String, env: &mut Env) -> Result<String, Errors>{
    let ast = READ(input)?;
    let result = EVAL(ast , env)?;
    Ok(PRINT(result))
}


