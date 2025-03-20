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
        loop {
            // Going through each line
       
            // using line_col_num and len of line to keep track of curr_line
            if self.curr_col_num > self.curr_line.len() {
                self.curr_line_num += 1;
                self.curr_col_num = 1;
                self.curr_line = self.contents.next();
            }

            // Logic for processing all chars in the current line:
            // Starts at the current col_num being processed
            let mut chars = self.curr_line[self.curr_col_num..].chars();





            // matching for the chars in line
            // possibly reusable for parts
            let mut chars = self.curr_line.chars();
            for c in chars {
                match c {
                    // case for left paren
                    '(' => {
                        let token = Token { 
                            lexeme: Lexeme::Lparen,
                            metadata: Metadata{
                                line_num: self.curr_line_num,
                                col_num: self.curr_col_num,
                            },
                        };
                        self.curr_col_num += 1;
                        return Some(token);
                    }

                    // case for right paren
                    ')' => {
                        let token = Token {
                            lexeme: Lexeme::Rparen,
                            metadata: Metadata {
                                line_num: self.curr_line_num,
                                col_num: self.curr_col_num,
                            },
                        };

                        self.curr_col_num += 1;
                        return Some(token);
                    }

                    // case for whitespace
                    c.is_whitespace() => {
                        self.curr_col_num += 1;
                        continue;
                    }

                    // case for atom
                    _ => {
                        let mut atom = String::new();
                        let start_col_num = self.curr_col_num;

                        // add each char to atom and build it

                        // returns the complete atom after detecting a whitespace or paren
                        // maybe an inner loop inside this case to complete the atom first
                        // then return whole atom
                        atom.push(c);


                        // go onto the next char inside the inner loop
                        next_char = chars.next();


                        // loop going through char still needs to be made
                        todo!  
                        while (next_char is not(whitespace, lparen, or rparen)) {
                            // add it to atom
                            // increment col space num
                            // or add the len of atom to the curr_col_num after the loop
                            // other solution to use a start_col num value as the col_num being returned
                        }

                        // returning the atom
                        return Some(Token {
                            lexeme: Lexeme::Atom(atom),
                            metadata: Metadata {
                                line_num = self.curr_line_num;
                                col_num = start_col_num;
                            }
                        });
                    }
                }
                self.curr_col_num += 1;
            }
        }
    }
}