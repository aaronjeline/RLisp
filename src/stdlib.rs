
pub fn get_std_lib() -> Vec<String> {
    let strs = vec![
        "(define load-file (fn (f) (eval (read (str \"(do \" (slurp f) \")\")))))",
        "(define not (fn (b) (if b false true)))"];
    strs.iter().map(|s| String::from(*s)).collect()
        
}
