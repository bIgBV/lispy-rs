extern crate lalrpop_util;
extern crate slotmap;

mod parser;

pub mod ast;

use parser::LispyParser;
use slotmap::SlotMap;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SyntaxError {
    ParseError,
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error during parsing")
    }
}

pub struct Parser {
    pub table: SlotMap<String>,
    parser: LispyParser,
}

impl Parser {
    pub fn new(table: SlotMap<String>) -> Self {
        Parser {
            table,
            parser: LispyParser::new(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<ast::Expr, SyntaxError> {
        match self.parser.parse(&mut self.table, input) {
            Ok(v) => Ok(ast::Expr::Sexp(v)),
            Err(_v) => Err(SyntaxError::ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut parser = Parser::new(SlotMap::new());
        assert!(parser.parse("+ 5 8 (* 10 9)").is_ok());
    }

    #[test]
    fn sexpr_test() {
        let mut parser = Parser::new(SlotMap::new());
        assert!(parser.parse("+ 9 (8 37 8)").is_ok());
    }

    #[test]
    fn qexpr_test() {
        let mut parser = Parser::new(SlotMap::new());
        assert!(parser.parse("+ 9 (8 37 8) {9 8 (3 8)}").is_ok());
    }

    #[test]
    fn builtin_test() {
        let mut parser = Parser::new(SlotMap::new());
        assert!(parser.parse("eval (tail {tail tail {5 6 7}})").is_ok());
    }
}
