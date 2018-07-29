extern crate rustyline;
extern crate syntax;

mod operator;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use syntax::ast::Lispy;
use syntax::ast::{Expr, Symbol};
use syntax::Parser;

use std::error;
use std::fmt;

use operator::Operate;

pub fn parse_input(input: &str) -> Result<Lispy, String> {
    match Parser::new().parse(input) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn read_input(rl: &mut Editor<()>) -> Result<String, ReadlineError> {
    let readline = rl.readline("lispy> ");

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_ref());
            Ok(line)
        }
        Err(e) => Err(e),
    }
}

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

impl error::Error for LispyError {
    fn description(&self) -> &str {
        "REPL error"
    }
}

pub type EvalResult<T> = std::result::Result<T, LispyError>;

/// Executes a builtin op. Right now only arithmetic opereations` TODO: Make this generic over Symbol<T> where T: Operate
pub fn builtin_op<T: Operate>(exprs: &Vec<Expr>, op: &T) -> EvalResult<Expr> {
    op.operate(exprs)
}

/// Main evaluation of our REPL process. The function iterates over all the expressions in an
/// S-expression and then calls eval_input on them. This is recursive step. Once all the sub
/// expressions have been evaluated, it handles the resulting list appropriately.
///
/// This function effectively walks down the AST, breaking it into individual blocks separated by
/// the symbol for a group and walks back up by combining the symbol and its operands until only
/// a single expression is left
fn eval(exprs: &Vec<Expr>) -> EvalResult<Expr> {
    let mut updated_exp = vec![];

    for expr in &*exprs {
        let result = try!(eval_input(expr));

        // Ignore an empty expression, effectively deleting it from the AST
        match result {
            Expr::Empty => {
                continue;
            }
            _ => updated_exp.push(result),
        }
    }

    // Empty expression will be removed down the recursive stack
    if updated_exp.len() == 0 {
        return Ok(Expr::Empty);
    }

    // A single expression such as (5)
    if updated_exp.len() == 1 {
        // TODO: should we allow single symbols?
        return Ok(updated_exp.remove(0));
    }

    // If the first item is a symbol in the list, then perform the operation for the symbol
    if let Expr::Sym(ref sym) = updated_exp[0] {
        return builtin_op(&updated_exp, sym);
    }

    unreachable!();
}

/// First level of expression evaluation. This is a simple match expression which either returns
/// a single expression or calls `eval` on an sexpression
fn eval_input(expr: &Expr) -> EvalResult<Expr> {
    match *expr {
        Expr::Val(v) => Ok(Expr::Val(v)),
        Expr::Sym(ref v) => match *v {
            Symbol::Arith(v) => Ok(Expr::Sym(Symbol::Arith(v))),
            Symbol::Builtin(v) => Ok(Expr::Sym(Symbol::Builtin(v))),
        },
        Expr::Sexp(ref v) => eval(v),
        Expr::Qexp(_) => Ok(expr.clone()), // TODO: Actual qexp handling
        Expr::Empty => Ok(Expr::Empty),
    }
}

fn main() {
    println!("Lispy Version 0.0.1");
    println!("Press Ctrl+C to exit");

    let mut rl = Editor::<()>::new();

    if rl.load_history("/tmp/lispy.history").is_err() {
        println!("No previous history file");
    }

    loop {
        let input = match read_input(&mut rl) {
            Ok(v) => v,
            Err(e) => {
                println!("Got error: {:?}", e);
                break;
            }
        };

        let parsed_val = match parse_input(&input) {
            Ok(v) => v,
            Err(e) => {
                println!("Got error: {:?}", e);
                continue;
            }
        };
        println!("lispy> {:?}", parsed_val);

        let value = match eval_input(&Expr::Sexp(parsed_val)) {
            Ok(v) => v,
            Err(e) => {
                println!("Got error: {:?}", e);
                continue;
            }
        };

        println!("lispy> {:?}", value);
    }

    println!("Exiting");

    rl.save_history("/tmp/lispy.history").unwrap();
}
