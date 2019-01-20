#[macro_use]
extern crate nom;
mod builtins;
mod parsing;
use nom::types::CompleteStr as Input;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use rlisp::*;
use crate::builtins::*;
use crate::parsing::*;
    
fn main() {
    let stdin = io::stdin();
    let env = build_init_env();

    loop {
        let mut buffer = String::new();
        print!("user>");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();
        if buffer.len() == 0 {
            break;
        }
        match rep(buffer, &env) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Runtime Error: {:?}", e),
        }
    }
    
}


fn build_init_env() -> Env {
    let mut l = HashMap::new();
    l.insert(String::from("+"), Value::Function(plus));
    l.insert(String::from("*"), Value::Function(mult));
    vec![l]
}


    

#[allow(non_snake_case)]
fn READ(input: String) -> Result<RValue, Errors>{
    match parse_value(Input(&input)) {
        Ok(v) => Ok(v.1),
        Err(_) => Err(Errors::ParseError),
    }
}

#[allow(non_snake_case)]
fn EVAL(input:RValue, env: &Env) -> FResult{

    match *(eval_ast(*input, env))? {
        Value::List(lst) =>
            if lst.len() == 0 {
                Ok(Box::new(Value::List(lst)))
            } else {
                function_call(lst)
            },
        other => Ok(Box::new(other)),
    }

}

fn function_call(lst: LinkedList<RValue>) -> FResult {
    let mut list = lst;

    let value = match list.pop_front() {
        Some(b) => *b,
        None => panic!("Empty list pased to function_call()"),
    };

    let f = match value {
        Value::Function(f) => Ok(f),
        _ => Err(Errors::NotAFunction),
    }?;

    f(list)
}

fn eval_ast(input:Value, env: &Env) -> FResult{
    match input {
        Value::Symbol(s) => lookup(s, env),
        Value::List(lst) => eval_list(lst, env),
        other => Ok(Box::new(other))
    }
}

fn eval_list(lst: LinkedList<RValue>, env: &Env) -> FResult {
    let mut new = LinkedList::new();
    for val in lst {
        let evald = EVAL(val, env)?;
        new.push_back(evald);
    }
    Ok(Box::new(Value::List(new)))
}



#[allow(non_snake_case)]
fn PRINT(input: RValue) -> String{
    input.to_string()
}

fn rep(input: String, env: &Env) -> Result<String, Errors>{
    let ast = READ(input)?;
    let result = EVAL(ast , env)?;
    Ok(PRINT(result))
}


