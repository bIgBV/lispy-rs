use std::fmt::{Debug, Error, Formatter};

pub type Number = f64;

#[derive(Copy, Clone)]
pub enum Arith {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Debug for Arith {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Arith::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mod => write!(fmt, "%"),
        }
    }
}

#[derive(Debug)]
pub enum Symbol {
    Arith(Arith),
}

pub type Sexpr = Vec<Box<Expr>>;

pub enum Expr {
    Val(Number),
    Sym(Symbol),
    Sexp(Vec<Box<Expr>>),
    Qexp(Vec<Box<Expr>>),
    Empty,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;

        match *self {
            Val(v) => write!(fmt, "{:?}", v),
            Sym(ref s) => write!(fmt, "{:?}", s),
            Sexp(ref e) => write!(fmt, "{:?}", e),
            Qexp(ref e) => write!(fmt, "{{ {:?} }}", e),
            Empty => write!(fmt, ""),
        }
    }
}

pub type Lispy = Vec<Box<Expr>>;
