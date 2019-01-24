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

    pub fn add_all(&mut self, ss: LinkedList<String>, vs: LinkedList<RValue>) -> Result<(),Errors> {
        if ss.len() != vs.len() {
            return Err(Errors::ArityError(ss.len() as i32, vs.len() as i32));
        } 
        
        for (symbol, value) in ss.iter().zip(vs) {
            self.set(symbol.to_string(), (*value).clone());
        }
        return Ok(());
    }




}

#[derive(Debug)]
pub enum Errors {
    TypeError,
    ParseError (String),
    SymbolNotFound (String),
    NotAFunction,
    FormError,
    ArityError (i32, i32),
    IOError (String),
}

#[derive(Clone, Debug, PartialEq)]
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
    Do,
    If,
    Fn,
    Eval,
    Quote,
    Function (fn(Params) -> FResult),
    DynFunc (DynamicFunction),
}

impl Value {
    pub fn equals(a: &Value, b: &Value) -> Result<bool, Errors> {
        
        match (a, b) {
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Str(a), Value::Str(b)) => Ok(a == b),
            (Value::True, Value::True) => Ok(true),
            (Value::False, Value::False) => Ok(true),
            (Value::True, Value::False) => Ok(false),
            (Value::False, Value::True) => Ok(false),
            (Value::List(a), Value::List(b)) => {
                if a.len() == b.len() {
                    a.iter().zip(b)
                        .fold(Ok(true),
                              |rest, (a,b)|
                              match rest {
                                  Err(e) => Err(e),
                                  Ok(r) => Ok(r && Value::equals(a, b)?),
                              })
                } else {
                    Ok(false)
                }
            }
            _ => Err(Errors::TypeError),
        }
    }
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
            Value::Function (_) =>  String::from("<Builtin-Function>"),
            Value::List (lst) => list_to_string(lst),
            Value::Define => String::from("define"),
            Value::Let => String::from("let"),
            Value::Do => String::from("do"),
            Value::If => String::from("if"),
            Value::Fn => String::from("fn"),
            Value::DynFunc (_) => String::from("<Function>"),
            Value::Eval => String::from("eval"),
            Value::Quote => String::from("quote"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicFunction {
    pub parameters: LinkedList<String>,
    pub body: RValue,
}

impl DynamicFunction {

    pub fn new(parameters: LinkedList<String>, body: RValue) -> DynamicFunction {
        DynamicFunction { parameters : parameters, body: body }
    }

}
fn list_to_string(lst: &LinkedList<Box<Value>>) -> String {
    let empty = String::from("");
    let contents = lst.iter().fold(empty,
                                   |s, next| format!("{} {}", s, next.to_string()));
    format!("({})", contents)
}


#[allow(non_snake_case)]
pub fn PRINT(input: RValue) -> String{
    input.to_string()
}

