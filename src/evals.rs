use rlisp::*;
use std::collections::LinkedList;
use std::ops::Deref;

#[allow(non_snake_case)]
pub fn EVAL(input:RValue, env: &mut Env) -> FResult{
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

pub fn eval_list(input: LinkedList<RValue>, env: &mut Env) -> FResult{
    let list = input;

    let first = match list.front() {
        Some(b) => b,
        None => panic!("Empty list in eval_list()"),
    };

    // Keyword detection here
    match first.deref() {
        Value::Define => eval_define(list, env),
        Value::Let => eval_let(list, env),
        Value::Do => eval_do(list, env),
        _ => function_call(recr_eval_list(list, env)?),
    }

}

fn eval_let(list: LinkedList<RValue>, env: &mut Env) -> FResult{
    let mut list = list;
    list.pop_front(); // Remove 'let'
    // Extract the binding term
    let binding = match list.pop_front() {
        Some(v) => Ok(v),
        None => Err(Errors::FormError),
    }?;
    let mut binding = match binding.deref().clone() {
        Value::List(lst) => Ok(lst),
        _ => Err(Errors::FormError),
    }?;
    binding.push_front(Box::new(Value::Define)); // Bush define to the front
    // Create a new scope
    let mut scope = Env::push((*env).clone());
    // Evaluate the binding in the new scope
    eval_define(binding, &mut scope)?;
    // Evaluate body in the new scope
    let body = match list.pop_front() {
        Some(body) => Ok(body),
        None => Err(Errors::FormError),
    }?;
    EVAL(body, &mut scope)
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

fn eval_do(list: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = list;
    list.pop_front(); // remove `do`
    let mut result = Ok(Box::new(Value::Nil));
    for expr in list {
        result = EVAL(expr, env);
    }
    result
}
    
