use std::error::Error;
use std::fmt;
use std::result::Result;

struct LispyError {
    kind: ErrorKind,
    message: &'static str,
}

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
        match *self.kind {
            ErrorKind::BadOp
            | ErrorKind::BadNum
            | ErrorKind::BadOperand
            | ErrorKind::EvalError
            | ErrorKind::ListError
            | ErrorKind::ParseError
            | ErrorKind::TypeError => write!(f, "{:?}: {}", *self.kind, *self.message),
        }
    }
}

impl Error for LispyError {}

pub fn make_error(kind: ErrorKind, message: &str) -> LispyError {
    LispyError { kind, message }
}

pub type EvalResult<T> = Result<T, LispyError>;
