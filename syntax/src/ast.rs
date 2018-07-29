use std::fmt::{Debug, Error, Formatter};

pub type Number = f64;

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Builtin {
    Head,
    Tail,
    List,
    Join,
    Eval,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Arith(Arith),
    Builtin(Builtin),
}

pub type Sexpr = Vec<Expr>;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Val(Number),
    Sym(Symbol),
    Sexp(Vec<Expr>),
    Qexp(Vec<Expr>),
    Empty,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;

        match *self {
            Val(v) => write!(fmt, "{:?}", v),
            Sym(ref s) => write!(fmt, "{:?}", s),
            Sexp(ref e) => write!(fmt, "Sexp({:?})", e),
            Qexp(ref e) => write!(fmt, "Qexp{{{:?}}}", e),
            Empty => write!(fmt, ""),
        }
    }
}

pub type Lispy = Vec<Expr>;
