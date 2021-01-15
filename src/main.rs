use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let argc = args.len();

    if argc > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    }
    else if argc == 2 {
        run_file( &args[1]);
    }
    else {
        run_prompt();
    }
}

fn run_file(file: &str) {
    let code = std::fs::read_to_string(file).expect("Invalid filename");

    run( &code);
}

fn run_prompt() {
    use rustyline::{Editor, error::ReadlineError};

    let mut rl = Editor::<()>::new();
    
    //add history support later

    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) => {
                run(&line);
            },
            Err(_) => {
                println!("Quitting...");
                break;
            }
        }
    }
}

fn run(source: &str) {
    println!("{}", source);
}
