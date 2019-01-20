use rlisp::*;


pub fn plus(p: Params) -> FResult {
    let mut sum = 0;
    for b in p {
        match *b {
            Value::Int (i) => sum += i,
            _ => return Err(Errors::TypeError),
        }
    }
    Ok(Box::new(Value::Int(sum)))
}

pub fn mult(p: Params) -> FResult {
    let mut product = 1;
    for b in p {
        match *b {
            Value::Int (i) => product *= i,
            _ => return Err(Errors::TypeError),
        }
    }
    Ok(Box::new(Value::Int(product)))
}
