#[macro_use]
extern crate nom;
use nom::types::CompleteStr as Input;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;
use std::io::Write;

type RValue = Box<Value>;
type Env = Vec<HashMap<String, Value>>;
type Params = LinkedList<RValue>;
type FResult = Result<RValue, Errors>;

#[derive(Debug)]
enum Errors {
    TypeError,
    ParseError,
    SymbolNotFound (String),
    NotAFunction,
}

#[derive(Clone)]
enum Value {
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

fn lookup(symbol: String, env:&Env) -> FResult{
    for scope in env {
        match scope.get(&symbol) {
            Some(v) => return Ok(Box::new((*v).clone())),
            None => (),
        }
    }
    Err(Errors::SymbolNotFound(symbol.clone()))
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

fn build_init_env() -> Env {
    let mut l = HashMap::new();
    l.insert(String::from("+"), Value::Function(plus));
    l.insert(String::from("*"), Value::Function(mult));
    vec![l]
}


fn determinte_symbol(s: String) -> Value {
    match s.as_ref() {
        "true" => Value::True,
        "false" => Value::False,
        "nil" => Value::Nil,
        _ => Value::Symbol(s)
    }
}
    

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '+' || c == '-' || c == '*'
}


named!(parse_value<Input, RValue>,
       map!(alt!(parse_string   |
                 parse_symbol   |
                 parse_int      |
                 parse_list), |v| Box::new(v)));

named!(parse_list<Input, Value>, ws!(do_parse!(
    _start: tag!("(") >>
        contents: fold_many0!( parse_value, LinkedList::new(),
                               |mut ll: LinkedList<_>, item| {
                                   ll.push_back(item);
                                   ll 
                               }) >> 
    _end: tag!(")") >>
    (Value::List(contents)))));

named!(parse_int<Input, Value>, ws!(do_parse!(
    num: map_res!(take_while!(is_digit), 
                      |Input(s)| s.parse::<i32>()) >>
        (Value::Int(num)))));
            

named!(parse_string<Input, Value>, ws!(do_parse!(
            _start: tag!("\"") >>
            contents: map!(take_until!("\""), |Input(s)| String::from(s)) >>
            _end: tag!("\"") >>
            (Value::Str(contents.into())))));

named!(parse_symbol<Input, Value>, 
       ws!(map!(take_while1!(is_alpha),
                |Input(s)| determinte_symbol(s.to_string()))));



fn READ(input: String) -> Result<RValue, Errors>{
    match parse_value(Input(&input)) {
        Ok(v) => Ok(v.1),
        Err(_) => Err(Errors::ParseError),
    }
}

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



fn PRINT(input: RValue) -> String{
    input.to_string()
}

fn rep(input: String, env: &Env) -> Result<String, Errors>{
    let ast = READ(input)?;
    let result = EVAL(ast , env)?;
    Ok(PRINT(result))
}


fn main() {
    let stdin = io::stdin();
    let env = build_init_env();

    loop {
        let mut buffer = String::new();
        print!("user>");
        io::stdout().flush();
        stdin.read_line(&mut buffer);
        if buffer.len() == 0 {
            break;
        }
        match rep(buffer, &env) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Runtime Error: {:?}", e),
        }
    }
    
}
