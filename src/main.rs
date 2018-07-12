extern crate rustyline;
extern crate syntax;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use syntax::ast;
use syntax::ast::Lispy;
use syntax::Parser;

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

fn eval(op: ast::Operator, exprs: Vec<Box<ast::Expr>>) -> ast::Number {
    let mut nums = vec![];

    for exp in exprs {
        let val = match *exp {
            ast::Expr::Val(num) => num,
            ast::Expr::Group(op, exprs) => eval(op, exprs),
        };
        nums.push(val);
    }

    let init_val: ast::Number = match op {
        ast::Operator::Add => 0.0,
        ast::Operator::Sub => 0.0,
        ast::Operator::Mul => 1.0,
        ast::Operator::Div => 1.0,
        ast::Operator::Mod => 1.0,
    };

    nums.iter().fold(init_val, |acc, x| match op {
        ast::Operator::Add => acc + x,
        ast::Operator::Sub => acc - x,
        ast::Operator::Mul => acc * x,
        ast::Operator::Div => acc / x,
        ast::Operator::Mod => acc % x,
    })
}

fn eval_input(terms: Lispy) -> ast::Number {
    match terms {
        Lispy::Terms(op, exprs) => eval(op, exprs),
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

        let value = eval_input(parsed_val);

        println!("lispy> {}", value);
    }

    println!("Exiting");

    rl.save_history("/tmp/lispy.history").unwrap();
}
