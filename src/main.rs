extern crate lalrpop_util;
extern crate rustyline;
extern crate syntax;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use syntax::Lispy;
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

fn main() {
    println!("Lispy Version 0.0.1");
    println!("Press Ctrl+C to exit");

    let mut rl = Editor::<()>::new();

    if rl.load_history("/tmp/lispy.history").is_err() {
        println!("No previous history file");
    }

    {
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

            println!("parsed: {:?}", parsed_val);
        }
    }

    println!("Exiting");

    rl.save_history("/tmp/lispy.history").unwrap();
}
