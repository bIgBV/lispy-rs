extern crate lalrpop_util;

pub mod parser;

mod ast;

pub use parser::LispyParser as Parser;
pub use ast::Lispy as Lispy;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(parser::LispyParser::new().parse("+ 5 8 (* 10 9)").is_ok());
    }
}
