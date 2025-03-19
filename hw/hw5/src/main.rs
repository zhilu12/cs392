fn main() {
    println!("Hello, world!");
}


struct Metadata {
    line_num: usize,
    col_num: usize,
}

enum Sexpr<T> {
    Atom(T),
    List(Box<Vec<Sexpr<T>>>),
}

enum ParseErrorKind {
    NotClosed,
    ExtraClosed,
}

struct ParseError {
    kind: ParseErrorKind,
    metadata: Metadata,
}


impl Sexpr<String> {
    fn parse(input: &str) -> Result<Vec<Sexpr<String>>, ParseError> {
        todo!()
        // parses a string into a list of s-expressions
        // possibly uses lexer to tokenize the input
        // can goes through input and parse based on the rules
    }
}


enum Lexeme {
    Lparen,
    Rparen,
    Atom(String),
}

struct Token {
    lexeme: Lexeme,
    metadata: Metadata,
}


struct Lexer<'a> {
    contents: Lines<'a>,
    curr_line_num: usize,
    curr_col_num: usize,
    curr_line: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            contents: input.lines(),
            curr_line_num: 0,
            curr_col_num: 1,
            curr_line: "",
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // go through each line of contents and tokenize based on rules

        // keep track of line and col number

        // Going through each line
        while let Some(line) = self.contents.next() {
            self.curr_line_num += 1;
            self.curr_col_num = 1;
            self.curr_line = line;



            // matching for the chars in line

            let mut chars = self.curr_line.chars();
            for c in chars {
                match c {
                    // case for left paren
                    '(' => {

                    }

                    // case for right paren
                    ')' => {
                        
                    }

                    // case for whitespace
                    c.is_whitespace() => {
                        self.curr_col_num += 1;
                        continue;
                    }

                    // case for atom
                    _ => {
                        
                    }
                }
            }
        }
    }
}