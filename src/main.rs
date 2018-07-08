extern crate rustyline;
#[macro_use]
extern crate nom;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parser;

fn main() {
    println!("Lispy Version 0.0.1");
    println!("Press Ctrl+C to exit");

    let mut rl = Editor::<()>::new();

    if rl.load_history("/tmp/lispy.history").is_err() {
        println!("No previous history file");
    }

    loop {
        let readline = rl.readline("lispy> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                println!("No you're a {:?}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting");
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        };
    }

    rl.save_history("/tmp/lispy.history").unwrap();
}
