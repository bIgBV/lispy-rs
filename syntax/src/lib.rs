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
}