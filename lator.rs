
mod scanner;
mod parser;
mod interpreter;

macro_rules! prompt(
    ($($arg:tt)*) => { {
        print!($($arg)*);
        stdout().flush().expect("Could not flush stdout");
    } }
);

fn run_prompt() {
    use std::io::{Write, BufRead, stdin, stdout};
    use interpreter::Interpreter;
    use std::collections::HashMap;

    let mut symbols: HashMap<String, i64> = HashMap::new();

    prompt!("> ");
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let mut interpreter = Interpreter::new_continue(line.unwrap(), symbols);
        symbols = interpreter.interpret();
        interpreter.print_symbols();
        prompt!("> ");
    }
}

fn run_file(file_path: &String) {
    use std::fs::File;
    use std::io::Read;
    use interpreter::Interpreter;

    let mut file = File::open(file_path).expect("Could not open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read file into string");

    let mut interpreter = Interpreter::new(file_contents);
    interpreter.interpret();
    interpreter.print_symbols();
}

fn main() {
    use std::env;

    // Setup command line arguments array
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: ./lator [script]")
    }
}
