#[macro_use]
extern crate nom;
mod builtins;
mod parsing;
use nom::types::CompleteStr as Input;
use std::ops::Deref;
use std::collections::LinkedList;
use std::io;
use std::io::Write;
use rlisp::*;
use crate::builtins::*;
use crate::parsing::*;
    
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
fn EVAL(input:RValue, env: &mut Env) -> FResult{
    match *input {
        Value::List(lst) =>
            if lst.len() == 0 {
                Ok(Box::new(Value::List(lst)))
            } else{
                eval_list(lst, env)
            },
        other => eval_ast(other, env) 
    }

}

fn eval_list(input: LinkedList<RValue>, env: &mut Env) -> FResult{
    let list = input;

    let first = match list.front() {
        Some(b) => b,
        None => panic!("Empty list in eval_list()"),
    };

    match first.deref() {
        Value::Define => eval_define(list, env),
        _ => function_call(recr_eval_list(list, env)?),
    }

}

fn eval_define(list: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = list;
    list.pop_front(); // Remove 'define'
    let name = match list.pop_front() {
        Some(v) => match *v {
            Value::Symbol(s) => Ok(s),
            _ => Err(Errors::TypeError),
        }
        _ => panic!("Empty list in define()"),
    }?;
    let target = match list.pop_front() {
        Some(b) => Ok(b),
        None => Err(Errors::FormError),
    }?;
    let target = EVAL(target, env)?;

    env.set(name, *target);
    Ok(Box::new(Value::Nil))
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

fn eval_ast(input:Value, env: &mut Env) -> FResult{
    match input {
        Value::Symbol(s) => env.lookup(s),
        other => Ok(Box::new(other))
    }
}

fn recr_eval_list(lst: LinkedList<RValue>, env: &mut Env)
                  -> Result<LinkedList<RValue>, Errors>{
    let mut new = LinkedList::new();
    for val in lst {
        let evald = EVAL(val, env)?;
        new.push_back(evald);
    }
    Ok(new)
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


