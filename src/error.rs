use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::result::Result;

#[derive(Debug)]
pub struct LispyError<T>
where
    T: ErrorKind + Debug,
{
    kind: T,
    message: String,
}

pub trait ErrorKind {}

#[derive(Debug)]
pub enum ProgramError {
    BadArgs,
    BadType,
}

impl ErrorKind for ProgramError {}

#[derive(Debug)]
pub enum LangError {
    EvalError,
    ParseError,
}

impl ErrorKind for LangError {}

impl<T> fmt::Display for LispyError<T>
where
    T: ErrorKind + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ref _t => write!(f, "{:?}: {:?}", self.kind, self.message),
        }
    }
}

impl<T> Error for LispyError<T> where T: ErrorKind + Debug {}

pub fn make_error<T: ErrorKind + Debug + 'static>(kind: T, message: String) -> Box<Error> {
    Box::new(LispyError { kind, message })
}

pub type EvalResult<T> = Result<T, Box<Error>>;
