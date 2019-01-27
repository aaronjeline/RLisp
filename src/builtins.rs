use rlisp::*;
use std::collections::LinkedList;
use std::ops::Deref;
use std::fs;
use crate::parsing::*;
use nom::types::CompleteStr as Input;

pub fn build_init_env() -> Env {
    let mut env = Env::new();
    env.set(String::from("+"), Value::Function(plus));
    env.set(String::from("*"), Value::Function(mult));
    env.set(String::from("prn"), Value::Function(prn));
    env.set(String::from("list"), Value::Function(list));
    env.set(String::from("list?"), Value::Function(list_p));
    env.set(String::from("empty?"), Value::Function(empty));
    env.set(String::from("count"), Value::Function(count));
    env.set(String::from("read"), Value::Function(read_str));
    env.set(String::from("="), Value::Function(equals));
    env.set(String::from("slurp"), Value::Function(slurp));
    env.set(String::from("str"), Value::Function(string));
    env.set(String::from("cons"), Value::Function(lisp_cons));
    env.set(String::from("first"), Value::Function(first));
    env.set(String::from("rest"), Value::Function(rest));
    env.set(String::from("mod"), Value::Function(modulo));
    env
}

fn plus(p: Params) -> FResult {
    let mut sum = 0;
    for b in p {
        match *b {
            Value::Int (i) => sum += i,
            _ => return Err(Errors::TypeError),
        }
    }
    Ok(Box::new(Value::Int(sum)))
}

fn mult(p: Params) -> FResult {
    let mut product = 1;
    for b in p {
        match *b {
            Value::Int (i) => product *= i,
            _ => return Err(Errors::TypeError),
        }
    }
    Ok(Box::new(Value::Int(product)))
}

fn modulo(p: Params) -> FResult {
    let mut list = p;
    let a = match list.pop_front() {
        Some(b) => match b.deref() {
            Value::Int(i) => Ok(i.clone()),
            _ => Err(Errors::TypeError),
        },
        None => Err(Errors::ArityError(0,2)),
    }?;

    let b = match list.pop_front() {
        Some(b) => match b.deref() {
            Value::Int(i) => Ok(i.clone()),
            _ => Err(Errors::TypeError),
        },
        None => Err(Errors::ArityError(1,2)),
    }?;

    Ok(Box::new(Value::Int(a % b)))
    
}


fn prn(p: Params) -> FResult {
    let toprint = match p.front() {
        Some(v) => Ok(v),
        None => Err(Errors::ArityError(0, 1)),
    }?;
    println!("{}", toprint.to_string());
    Ok(Box::new(Value::Nil))
}

fn list(p: Params) -> FResult {
    Ok(Box::new(Value::List(p)))
}

fn list_p(p: Params) -> FResult {
    let subject = match p.front() {
        Some(v) => Ok(v),
        None => Err(Errors::ArityError(0, 1)),
    }?;
    let result = match (*subject).deref() {
        Value::List(_) => Value::True,
        _ => Value::False,
    };

    Ok(Box::new(result))
}

fn empty(p: Params) -> FResult {
    let subject = match p.front() {
        Some(v) => Ok(v),
        None => Err(Errors::ArityError(0, 1)),
    }?;

    let lst = match (*subject).deref() {
        Value::List(lst) => Ok(lst),
        _ => Err(Errors::TypeError),
    }?;

    Ok(Box::new(if lst.len() == 0 {Value::True} else {Value::False}))
}

fn count(p: Params) -> FResult {
    let subject = match p.front() {
        Some(v) => Ok(v),
        None => Err(Errors::ArityError(0, 1)),
    }?;

    let lst = match (*subject).deref() {
        Value::List(lst) => Ok(lst),
        _ => Err(Errors::TypeError),
    }?;

    Ok(Box::new(Value::Int(lst.len() as i32)))

}

fn read_str(p: Params) -> FResult {
    let subject = match p.front() {
        Some(v) => Ok(v),
        None => Err(Errors::ArityError(0, 1)),
    }?;

    let s = match (*subject).deref() {
        Value::Str(s) => Ok(s),
        _ => Err(Errors::TypeError),
    }?;

    match parse_value(Input(&s)) {
        Ok(v) => Ok(v.1),
        _=> Err(Errors::TypeError),
    }
        
}

fn equals(p: Params) -> FResult {
    let mut list = p;
    let a = match list.pop_front() {
        Some(a) => Ok(a),
        None => Err(Errors::ArityError(0, 2)),
    }?;
    let b = match list.pop_front() {
        Some(b) => Ok(b),
        None => Err(Errors::ArityError(1, 2)),
    }?;

    native_to_lisp(Value::equals(a.deref(), b.deref())?)
}

fn native_to_lisp(b : bool) -> FResult {
    Ok(Box::new(if b { Value::True } else { Value::False } ))
}

fn slurp(p: Params) -> FResult {
    let filename = match p.front() {
        Some(s) => Ok(s),
        None => Err(Errors::ArityError(0, 1)),
    }?;
    let filename = match filename.deref() {
        Value::Str(s) => Ok(s),
        _ => Err(Errors::TypeError),
    }?;

    match fs::read_to_string(filename) {
        Ok(s) => Ok(Box::new(Value::Str(s))),
        Err(_) => Err(Errors::IOError(filename.clone())),
    }

}

fn string(p: Params) -> FResult {
    let strings:Result<LinkedList<String>, _> = p
        .into_iter()
        .map(|v| match v.deref() {
            Value::Str(s) => Ok(s.clone()),
            _ => Err(Errors::TypeError),
        }).collect();
    let strings = strings?;
    let string = strings
        .iter()
        .fold(String::from(""),
              |sofar, next|
              format!("{}{}", sofar, next));
    
    Ok(Box::new(Value::Str(string)))
}

fn lisp_cons(p: Params) -> FResult {
    if p.len() != 2 {
        return Err(Errors::ArityError(p.len() as i32, 2));
    }
    let mut p = p;
    let first = p.pop_front().unwrap();
    let rest = p.pop_front().unwrap();
        
    let new  = match rest.deref() {
        Value::List(lst) => Ok(cons(first.deref().clone(), lst.deref().clone())),
        _ => Err(Errors::TypeError),
    }?;

    Ok(Box::new(Value::List(new)))

}

fn cons(first: Value, rest: LinkedList<RValue>) -> LinkedList<RValue>{
    let mut n = LinkedList::new();
    n.push_front(Box::new(first));
    for v in rest {
        n.push_back(v);
    }
    
    n
}

fn first(p: Params) -> FResult {
    match p.front() {
        Some(b) => match b.deref() {
            Value::List(lst) => match lst.front() {
                Some(v) => Ok(v.clone()),
                None => Ok(Box::new(Value::Nil)),
            }
            _ => Err(Errors::TypeError),
        }

        None => Err(Errors::ArityError(0, 1))
    }
}

fn rest(p: Params) -> FResult {
    match p.front() {
        Some(b) => match b.deref() {
            Value::List(lst) => {
                let mut lst = lst.clone();
                lst.pop_front();
                Ok(Box::new(Value::List(lst)))   
            },
            _ => Err(Errors::TypeError),
        }
        _ => Err(Errors::ArityError(0,1)),
    }
        
}

    
