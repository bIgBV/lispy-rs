use std::fmt::{Debug, Error, Formatter};

pub type Number = f64;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mod => write!(fmt, "%"),
        }
    }
}

pub enum Expr {
    Val(Number),
    Group(Operator, Vec<Box<Expr>>),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Val(n) => write!(fmt, "{}", n.to_string()),
            Group(ref op, ref exprs) => write!(fmt, "{:?} {:?}", op, exprs),
        }
    }
}

#[derive(Debug)]
pub enum Lispy {
    Terms(Operator, Vec<Box<Expr>>),
}
