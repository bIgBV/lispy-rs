extern crate lalrpop_util;
extern crate slotmap;

mod parser;

pub mod ast;

use parser::LispyParser;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SyntaxError {
    ParseError(String),
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error during parsing")
    }
}

pub struct Parser {
    parser: LispyParser,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            parser: LispyParser::new(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<ast::Expr, SyntaxError> {
        match self.parser.parse(input) {
            Ok(v) => Ok(ast::Expr::Sexp(v)),
            Err(v) => Err(SyntaxError::ParseError(format!("{:?}", v).to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parser = Parser::new();
        assert!(parser.parse("+ 5 8 (* 10 9)").is_ok());
    }

    #[test]
    fn sexpr_test() {
        let parser = Parser::new();
        assert!(parser.parse("+ 9 (8 37 8)").is_ok());
    }

    #[test]
    fn qexpr_test() {
        let parser = Parser::new();
        assert!(parser.parse("+ 9 (8 37 8) {9 8 (3 8)}").is_ok());
    }

    #[test]
    fn builtin_test() {
        let parser = Parser::new();
        assert!(parser.parse("eval (tail {tail tail {5 6 7}})").is_ok());
    }

    #[test]
    fn variable_test() {
        let parser = Parser::new();
        match parser.parse("def {x} 100") {
            Ok(v) => println!("Result: {:?}", v),
            Err(e) => panic!("Err: {:?}", e),
        }
    }
}
