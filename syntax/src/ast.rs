use std::fmt::{Debug, Error, Formatter};

pub type Number = f64;

#[derive(Copy, Clone)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Debug for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Symbol::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mod => write!(fmt, "%"),
        }
    }
}

pub type Sexpr = Vec<Box<Expr>>;

pub enum Expr {
    Val(Number),
    Sym(Symbol),
    Sexp(Vec<Box<Expr>>),
    Empty,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;

        match *self {
            Val(v) => write!(fmt, "{:?}", v),
            Sym(s) => write!(fmt, "{:?}", s),
            Sexp(ref e) => write!(fmt, "{:?}", e),
            Empty => write!(fmt, ""),
        }
    }
}

pub type Lispy = Vec<Box<Expr>>;
