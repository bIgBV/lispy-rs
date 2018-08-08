use std::error::Error;
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub struct LispyError {
    kind: ErrorKind,
    message: String,
}

// TODO: Split into different types of errors and have a common trait which LispyError has
#[derive(Debug)]
pub enum ErrorKind {
    BadOp,
    BadNum,
    BadOperand,
    BadArgs,
    BadType,
    EvalError,
    ParseError,
}

impl fmt::Display for LispyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::BadOp
            | ErrorKind::BadNum
            | ErrorKind::BadOperand
            | ErrorKind::BadArgs
            | ErrorKind::BadType
            | ErrorKind::EvalError
            | ErrorKind::ParseError => write!(f, "{:?}: {:?}", self.kind, self.message),
        }
    }
}

impl Error for LispyError {}

pub fn make_error(kind: ErrorKind, message: String) -> LispyError {
    LispyError { kind, message }
}

pub type EvalResult<T> = Result<T, LispyError>;
