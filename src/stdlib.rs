
pub fn get_std_lib() -> Vec<String> {
    let strs = vec![
        "(define load-file (fn (f) (eval (read (str \"(do \" (slurp f) \")\")))))",
        "(define not (fn (b) (if b false true)))",
        "(define even (fn (x) (= (mod x 2) 0)))",
        "(define odd (fn (x) (not (even x))))",
        "(define foldr (fn (f b lst) (if (empty? lst) b (f (first lst) (foldr f b (rest lst))))))",
        "(define map (fn (f lst) (foldr (fn (x sofar) (cons (f x) sofar)) '() lst)))",

        //"(define map (fn (f lst) (if (empty? lst) '() (cons (f (first lst)) (map f (rest lst))))))", 
    ];
    strs.iter().map(|s| String::from(*s)).collect()
        
}
