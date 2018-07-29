extern crate lalrpop_util;

pub mod parser;

pub mod ast;

pub use parser::LispyParser as Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(parser::LispyParser::new().parse("+ 5 8 (* 10 9)").is_ok());
    }

    #[test]
    fn sexpr_test() {
        assert!(parser::LispyParser::new().parse("+ 9 (8 37 8)").is_ok());
    }

    #[test]
    fn qexpr_test() {
        assert!(parser::LispyParser::new().parse("+ 9 (8 37 8) {9 8 (3 8)}").is_ok());
    }

    #[test]
    fn builtin_test() {
        assert!(parser::LispyParser::new().parse("eval (tail {tail tail {5 6 7}})").is_ok());
    }
}
