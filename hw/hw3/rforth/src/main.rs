use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut config = Config::new();
    let stdin = io::stdin();

    println!("Type `bye` to exit");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        // read line from stdin
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let entry = line.trim();

        // if bye, exit
        if entry == "bye" {
            break;
        }

        // split line into elements by whitespace
        let words: Vec<&str> = entry.split_whitespace().collect();
        for word in words {
            // if a word is bye, exit
            if word == "bye" {
                return;
            }
            // convert &str to String before calling eval_word
            if let Err(e) = config.eval_word(word.to_string()) {
                println!("{}", e);
                break;
            }
        }
    }
}

type Program = Vec<Command>;
type Dict = HashMap<String, Program>;
type Stack = Vec<i32>;

enum State {
    Compiled,
    Interpreted,
}

struct Config {
    dict: Dict,
    stack: Stack,
    state: State,
    compiled: Program,
    compiled_word: Option<String>,
}

#[derive(Clone)]
enum Command {
    Push(i32),
    Drop,
    Swap,
    Print,
    PrintStack,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Config {
    fn new() -> Config {
        Config {
            state: State::Interpreted,
            compiled: Vec::new(),
            compiled_word: None,
            stack: Vec::new(),
            dict: HashMap::from([
                (String::from("drop"), vec![Command::Drop]),
                (String::from("swap"), vec![Command::Swap]),
                (String::from("."), vec![Command::Print]),
                (String::from(".s"), vec![Command::PrintStack]),
                (String::from("+"), vec![Command::Add]),
                (String::from("-"), vec![Command::Sub]),
                (String::from("*"), vec![Command::Mul]),
                (String::from("/"), vec![Command::Div]),
                (String::from("mod"), vec![Command::Mod]),
            ]),
        }
    }

    fn eval_word(&mut self, word: String) -> Result<(), String> {
        if let Some(program) = self.dict.get(&word) {
            let program = program.clone();
            for command in program {
                self.eval_command(command)?;
            }
            Ok(())
        } else if let Ok(n) = word.parse::<i32>() {
            self.eval_command(Command::Push(n))
        } else {
            Err(format!("undefined word: {}", word))
        }
    }

    fn eval_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::Push(n) => {
                self.stack.push(n);
                Ok(())
            }
            Command::Drop => {
                if let Some(top) = self.stack.pop() {
                    println!("{}", top);
                    Ok(())
                } else {
                    Err("stack underflower".to_string())
                }
            }
            Command::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 2, len - 1);
                    Ok(())
                } else {
                    Err("stack underflow".to_string())
                }
            }
            Command::Print => {
                if let Some(top) = self.stack.pop() {
                    println!("{}", top);
                    Ok(())
                } else {
                    Err("stack underflow".to_string())
                }
            }
            Command::PrintStack => {
                for val in &self.stack {
                    print!("{} ", val);
                }
                println!();
                Ok(())
            }
            Command::Add => {
                if self.stack.len() < 2 {
                    Err("stack underflow".to_string())
                } else {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                    Ok(())
                }
            }
            Command::Sub => {
                if self.stack.len() < 2 {
                    Err("stack underflow".to_string())
                } else {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                    Ok(())
                }
            }
            Command::Mul => {
                if self.stack.len() < 2 {
                    Err("stack underflow".to_string())
                } else {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                    Ok(())
                }
            }
            Command::Div => {
                if self.stack.len() < 2 {
                    Err("stack underflow".to_string())
                } else {
                    let b = self.stack.pop().unwrap();
                    if b == 0 {
                        return Err("division by zero".to_string());
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                    Ok(())
                }
            }
            Command::Mod => {
                if self.stack.len() < 2 {
                    Err("stack underflow".to_string())
                } else {
                    let b = self.stack.pop().unwrap();
                    if b == 0 {
                        return Err("division by zero".to_string());
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                    Ok(())
                }
            }
        }
    }

}




