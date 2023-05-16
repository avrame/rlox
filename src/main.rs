use std::env;
use std::fs;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();
    
    match args.len() {
        1 => {
            lox.run_prompt();
        },
        2 => {
            let path = args[1].clone();
            lox.run_file(path);
        },
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        },
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox {
            had_error: false,
        }
    }

    fn run_prompt(&mut self) {
        println!("Running prompt");
        
        loop {
            // get a string from stdin
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read line");
            if line.is_empty() {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    fn run_file(&mut self, path: String) {
        println!("Opening file: {}", path);
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");
        println!("With contents: \n{}", contents);
        if self.had_error {
            std::process::exit(65);
        }
    }

    fn run(&mut self, line: String) {
        println!("Running line: {}", line);
        
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: usize, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
