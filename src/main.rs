extern crate rustyline;
extern crate syntax;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use syntax::ast::Lispy;
use syntax::ast::{Expr, Number, Symbol};
use syntax::Parser;

use std::error;
use std::fmt;

fn parse_input(input: &str) -> Result<Lispy, String> {
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
enum LispyError {
    BadOp,
    BadNum,
}

impl fmt::Display for LispyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispyError::BadOp => write!(f, "Bad Symbol"),
            LispyError::BadNum => write!(f, "Bad Number"),
        }
    }
}

impl error::Error for LispyError {
    fn description(&self) -> &str {
        "REPL error"
    }
}

type EvalResult<T> = std::result::Result<T, LispyError>;

fn perform_artih_op(op: Symbol, lhs: Number, rhs: Number) -> Number {
    match op {
        Symbol::Add => lhs + rhs,
        Symbol::Sub => lhs - rhs,
        Symbol::Mul => lhs * rhs,
        Symbol::Div => lhs / rhs,
        Symbol::Mod => lhs % rhs,
    }
}

/// Main evaluation of our REPL process. The function iterates over all the expressions in an
/// S-expression and then calls eval_input on them. This is recursive step. Once all the sub
/// expressions have been evaluated, it handles the resulting list appropriately.
///
/// This function effectively walks down the AST, breaking it into individual blocks separated by
/// the symbol for a group and walks back up by combining the symbol and its operands until only
/// a single expression is left
fn eval(exprs: &Vec<Box<Expr>>) -> EvalResult<Expr> {
    let mut updated_exp = vec![];

    for expr in &*exprs {
        let result = try!(eval_input(expr));

        // Ignore an empty expression, effectively deleting it from the AST
        match result {
            Expr::Empty => continue,
            _ => updated_exp.push(Box::new(result)),
        }
    }

    // Empty expression will be removed down the recursive stack
    if exprs.len() == 0 {
        return Ok(Expr::Empty);
    }

    // A single expression such as (5)
    if updated_exp.len() == 1 {
        // TODO: should we allow single symbols?
        return Ok(*updated_exp.remove(0));
    }

    // If the first item is a symbol in the list, then perform the operation for the symbol
    if let Expr::Sym(sym) = *updated_exp[0] {
        return builtin_op(&updated_exp, sym);
    }

    unreachable!();
}

/// Executes a builtin op. Right now only arithmetic opereations`
// TODO: Make this generic over Symbol<T> where T: Operate
fn builtin_op(exprs: &Vec<Box<Expr>>, op: Symbol) -> EvalResult<Expr> {
    let init_val: Number = match op {
        Symbol::Add => 0.0,
        Symbol::Sub => 0.0,
        Symbol::Mul => 1.0,
        Symbol::Div => 1.0,
        Symbol::Mod => 1.0,
    };

    let mut acc = init_val;

    for x in &exprs[1..] {
        let val = match **x {
            Expr::Val(v) => v,
            _ => {
                return Err(LispyError::BadNum);
            }
        };
        acc = perform_artih_op(op, acc, val);
    }

    Ok(Expr::Val(acc))
}

/// First level of expression evaluation. This is a simple match expression which either returns
/// a single expression or calls `eval` on an sexpression
fn eval_input(expr: &Expr) -> EvalResult<Expr> {
    match *expr {
        Expr::Val(v) => Ok(Expr::Val(v)),
        Expr::Sym(v) => Ok(Expr::Sym(v)),
        Expr::Exp(ref v) => eval(v),
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

        let value = match eval_input(&Expr::Exp(parsed_val)) {
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
