fn main() {
    println!("Hello, world!");
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

    fn eval_word(word: String) {
        if let some(program) = self.dict.get(&word) {
            for command in program {
                self.eval_command(command);
            }
        }
    } else {
        if let Ok(n) = word.parse::<i32> {
            self.eval_command(Command::Push(n))
        } else {
            Err()
        }
    }

    fn eval_command(&mut self, command: Command) {
        match command {
            Command::Push(n) => {
                self.stack.push(n);
            }
            Command::Drop => {
                self.stack.pop();
            }
            Command::Swap => {

                let len = stack.len();
                self.stack.swap(len-2, len-1)
            }
            Command::Print => {
                if let Some(top) = stack.pop() {
                    println!("{}", top);
                } else {
                    Err("No elements on stack")
                }
            }
            Command::PrintStack => {
                if stack.len() > 0 {
                    for Some(n) in stack {
                        println!("{} ", n)
                    }
                } else {
                    Err("No elements on stack")
                }
            }
            Command::Add => {
                if let some(n),
            }
            Sub,
            Mul,
            Div,
            Mod,
        }
    }
}


