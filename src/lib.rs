use std::collections::LinkedList;
use std::collections::HashMap;

pub type RValue = Box<Value>;
pub type Params = LinkedList<RValue>;
pub type FResult = Result<RValue, Errors>;

#[derive(Clone)]
pub struct Env {
    contents: HashMap<String, Value>,
    past: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Env{
        Env { contents: HashMap::new(), past: None }
    }

    pub fn lookup(&self, symbol: String) -> FResult{
        match self.contents.get(&symbol) {
            Some(v) => Ok(Box::new((*v).clone())),
            None => match &self.past {
                Some(scope) => scope.lookup(symbol),
                None => Err(Errors::SymbolNotFound(symbol.clone()))
            }
        }
    }

    pub fn set(&mut self, symbol: String, v: Value) {
        self.contents.insert(symbol, v);
    }

    pub fn push(old: Env) -> Env {
        let mut n = Env::new();
        n.past = Some(Box::new(old));
        n
    }



}

#[derive(Debug)]
pub enum Errors {
    TypeError,
    ParseError,
    SymbolNotFound (String),
    NotAFunction,
    FormError,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int (i32),
    Str (String),
    Symbol (String),
    List (LinkedList<Box<Value>>),
    True,
    False,
    Nil,
    Define,
    Let,
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
            Value::Define => String::from("define"),
            Value::Let => String::from("Let")
        }
    }
}

fn list_to_string(lst: &LinkedList<Box<Value>>) -> String {
    let empty = String::from("");
    let contents = lst.iter().fold(empty,
                                   |s, next| format!("{} {}", s, next.to_string()));
    format!("({})", contents)
}


