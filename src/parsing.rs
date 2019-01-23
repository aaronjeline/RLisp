use rlisp::*;
extern crate nom;
use nom::types::CompleteStr as Input;
use std::collections::LinkedList;


fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '+' || c == '-' || c == '*' || c == '?' || c == '_'
        || c == '=' || c == '<' || c == '>'
}

fn determinte_symbol(s: String) -> Value {
    match s.as_ref() {
        "true" => Value::True,
        "false" => Value::False,
        "nil" => Value::Nil,
        "define" => Value::Define,
        "let" => Value::Let,
        "do" => Value::Do,
        "if" => Value::If,
        "fn" => Value::Fn,
        "eval" => Value::Eval,
        _ => Value::Symbol(s)
    }
}


named!(pub parse_value<Input, RValue>,
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
