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
        Value::Defmacro => eval_defmacro(list, env),
        Value::Let => eval_let(list, env),
        Value::Do => eval_do(list, env),
        Value::If => eval_if(list, env),
        Value::Fn => eval_fn(list),
        Value::Eval => eval_eval(list, env),
        Value::Quote => eval_quote(list),
        Value::Quasiquote => eval_qquote(list, env),
        _ => handle_function(recr_eval_list(list, env)?, env),
    }

}


fn eval_qquote(list: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = list;
    list.pop_front(); // Remove `qquote`
    let target = list.pop_front().unwrap();
    recr_qquote(target, env)
}

fn recr_qquote(val: RValue, env: &mut Env) -> FResult {
    match val.deref() {
        Value::List(list) => {
            let list = list.clone();
            if list.len() == 0 {
                Ok(Box::new(Value::List(list)))
            } else { // Check for unquotes
                let mut list = list;
                let first = list.front().unwrap();
                match first.deref() {
                    Value::Unquote => { // Do evaluate
                        list.pop_front();
                        let target = list.pop_front().unwrap();
                        EVAL(target, env)
                    },
                    // recurse to look for unquotes
                    _ => {
                        let checked: Result<LinkedList<RValue>, Errors>  = list
                            .into_iter()
                            .map(
                                |expr| recr_qquote(expr, env))
                            .collect();
                        Ok(Box::new(Value::List(checked?)))
                    }
                }
            }
        },
       v => Ok(Box::new(v.clone()))
    }
}

fn eval_quote(lst: LinkedList<RValue>) -> FResult {
    let mut list = lst;
    list.pop_front(); // Remove quote;
    match list.front() {
        Some(v) => Ok(v.clone()),
        None => Err(Errors::FormError),
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

fn eval_defmacro(list: LinkedList<RValue>, env: &mut Env) -> FResult {
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
    let target = match target.deref() {
        Value::DynFunc(df) => Ok(Value::DynFunc(to_macro(df.clone()))),
        _ => Err(Errors::TypeError),
    }?;

    env.set(name, target);
    Ok(Box::new(Value::Nil))
}

fn to_macro(f: DynamicFunction) -> DynamicFunction {
    DynamicFunction::new_macro(f.parameters, f.body)
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

fn handle_function(lst: LinkedList<RValue>, env: &mut Env) -> FResult {
    match lst.front() {
        Some(b) => match b.deref() {
            Value::Function(_) => handle_builtin(lst),
            Value::DynFunc(_) => handle_dyn_function(lst, env),
            _ => Err(Errors::FormError),
        }
        None => Err(Errors::FormError),
    }

}

fn handle_dyn_function(lst: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = lst;
    let f = match list.pop_front().unwrap().deref().clone() {
        Value::DynFunc(f) => Ok(f),
        _ => Err(Errors::FormError),
    }?;
    let mut scope = Env::push((*env).clone());
    scope.add_all(f.parameters, list)?;
    EVAL(f.body, &mut scope)
}

fn handle_builtin(lst: LinkedList<RValue>) -> FResult {
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
    
fn eval_if(list: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = list;
    list.pop_front(); // remove `if`
    if list.len() < 2 {
        return Err(Errors::FormError);
    }

    let cond = list.pop_front().unwrap();
    let a = list.pop_front().unwrap();
    let b = match list.pop_front() {
        Some(b) => b,
        None => Box::new(Value::Nil),
    };

    let cond = EVAL(cond, env)?;

    let result =
        if *cond == Value::True {
            a
        } else {
            b
        };

   EVAL(result, env) 
    
}

fn eval_fn(list: LinkedList<RValue>) -> FResult {
    let mut list = list;
    list.pop_front(); // Drop `fn`
    if list.len() < 2 {
        return Err(Errors::FormError);
    }
    let params = build_param_list(list.pop_front().unwrap())?;

    let body = list.pop_front().unwrap();
    let df = DynamicFunction::new(params, body);
    Ok(Box::new(Value::DynFunc(df)))
}

fn build_param_list(list: RValue) -> Result<LinkedList<String>,Errors> {
    match list.deref() {
        Value::List(list) => 
            Ok(list.iter()
            .map(|v| match v.deref() {
                Value::Symbol(s) => Some(s),
                _ => None, })
            .filter(|o| match o {
                Some(_) => true,
                None => false })
            .map(|o| match o {
                Some(s) => (*s).clone(),
                None => panic!(""),
            }).collect()),
            
            
        _ => Err(Errors::FormError),
    }
}

fn eval_eval(list: LinkedList<RValue>, env: &mut Env) -> FResult {
    let mut list = list;
    list.pop_front(); //Drop `eval`
    let target = match list.pop_front() {
        Some(b) => Ok(b),
        None => Err(Errors::ArityError(0, 1)),
    }?;
    let evald = EVAL(target, env)?;
    EVAL(evald, env)
}
