use std::collections::LinkedList;
use std::collections::HashMap;

pub type RValue = Box<Value>;
pub type Env = Vec<HashMap<String, Value>>;
pub type Params = LinkedList<RValue>;
pub type FResult = Result<RValue, Errors>;

#[derive(Debug)]
pub enum Errors {
    TypeError,
    ParseError,
    SymbolNotFound (String),
    NotAFunction,
}

#[derive(Clone)]
pub enum Value {
    Int (i32),
    Str (String),
    Symbol (String),
    List (LinkedList<Box<Value>>),
    True,
    False,
    Nil,
    Function (fn(Params) -> FResult),
}

impl ToString for Value {
    fn to_string(&self) -> String{
        match self {
            Value::Int (i) => i.to_string(),
            Value::Str (s) => format!("\"{}\"", s),
            Value::Symbol (s) => s.clone(),
            Value::True => String::from("True"),
            Value::False => String::from("False"),
            Value::Nil => String::from("Nil"),
            Value::Function (_) =>  String::from("Function"),
            Value::List (lst) => list_to_string(lst),
        }
    }
}

fn list_to_string(lst: &LinkedList<Box<Value>>) -> String {
    let empty = String::from("");
    let contents = lst.iter().fold(empty,
                                   |s, next| format!("{} {}", s, next.to_string()));
    format!("({})", contents)
}

pub fn lookup(symbol: String, env:&Env) -> FResult{
    for scope in env {
        match scope.get(&symbol) {
            Some(v) => return Ok(Box::new((*v).clone())),
            None => (),
        }
    }
    Err(Errors::SymbolNotFound(symbol.clone()))
}

