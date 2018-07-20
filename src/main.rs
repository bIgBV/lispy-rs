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

fn eval(exprs: &Vec<Box<Expr>>) -> EvalResult<Expr> {
    let mut updated_exp = vec![];

    for expr in &*exprs {
        let result = try!(eval_input(expr));
        match result {
            Expr::Empty => continue,
            _ => updated_exp.push(Box::new(result)),
        }
    }

    if exprs.len() == 0 {
        return Ok(Expr::Empty);
    }

    if updated_exp.len() == 1 {
        return Ok(*updated_exp.remove(0));
    }

    if let Expr::Sym(sym) = *updated_exp[0] {
        return builtin_op(&updated_exp, sym);
    }

    Err(LispyError::BadOp)
}

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
