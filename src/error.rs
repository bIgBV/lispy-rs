use std::error::Error;
use std::fmt;
use std::result::Result;

#[derive(Debug)]
pub enum LispyError {
    BadOp,
    BadNum,
    BadOperand,
    ListError(String),
}

impl fmt::Display for LispyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispyError::BadOp => write!(f, "Bad Symbol"),
            LispyError::BadNum => write!(f, "Bad Number"),
            LispyError::BadOperand => write!(f, "Bad Operand"),
            LispyError::ListError(ref v) => write!(f, "{}", v),
        }
    }
}

impl Error for LispyError {
    fn description(&self) -> &str {
        "REPL error"
    }
}

pub type EvalResult<T> = Result<T, LispyError>;
