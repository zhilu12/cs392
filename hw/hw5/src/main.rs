use std::str::Lines;
use std::iter::Peekable;

fn main() {
    let input = r#"(add
(mul 2 3)
4)
((one "1") (two "2"))"#;
  
    match Sexpr::parse(input) {
        Ok(sexprs) => {
            for sexpr in sexprs {
                println!("{:#?}", sexpr);
            }
        }
        Err(err) => {
            println!(
                "Parse error: {:?} at line {}, col {}",
                err.kind, err.metadata.line_num, err.metadata.col_num
            );
        }
    }
}

#[derive(Debug, Clone)]
struct Metadata {
    line_num: usize,
    col_num: usize,
}

#[derive(Debug)]
enum Sexpr<T> {
    Atom(T),
    List(Box<Vec<Sexpr<T>>>),
}

#[derive(Debug)]
enum ParseErrorKind {
    NotClosed,
    ExtraClosed,
}

#[derive(Debug)]
struct ParseError {
    kind: ParseErrorKind,
    metadata: Metadata,
}


impl Sexpr<String> {
    fn parse(input: &str) -> Result<Vec<Sexpr<String>>, ParseError> {
        let mut tokens = Lexer::new(input).peekable();
        let mut sexprs = Vec::new();

        // parse a single S-expression
        fn parse_expr<I>(tokens: &mut Peekable<I>) -> Result<Sexpr<String>, ParseError>
        where
            I: Iterator<Item = Token>,
        {
            // consume the next token
            let token = tokens.next().ok_or(ParseError {
                kind: ParseErrorKind::NotClosed,
                metadata: Metadata {
                    line_num: 0, 
                    col_num: 0
                },
            })?;
            match token.lexeme {
                Lexeme::Atom(s) => Ok(Sexpr::Atom(s)),
                Lexeme::Lparen => {
                    let mut children = Vec::new();
                    loop {
                        match tokens.peek() {
                            // finish list if there is a right paren
                            Some(peek_token) if matches!(peek_token.lexeme, Lexeme::Rparen) => {
                                tokens.next();
                                break Ok(Sexpr::List(Box::new(children)));
                            }
                            // if tokens run out
                            None => {
                                return Err(ParseError {
                                    kind: ParseErrorKind::NotClosed,
                                    metadata: token.metadata,
                                });
                            }
                            // parsing expression
                            _ => {
                                let expr = parse_expr(tokens)?;
                                children.push(expr);
                            }
                        }
                    }
                }
                // unexpected right paren
                Lexeme::Rparen => Err(ParseError {
                    kind: ParseErrorKind::ExtraClosed,
                    metadata: token.metadata,
                }),
            }
        }

        // parse expressions until input is exhausted
        while let Some(peeked) = tokens.peek() {
            if let Lexeme::Rparen = peeked.lexeme {
                return Err(ParseError {
                    kind: ParseErrorKind::ExtraClosed,
                    metadata: peeked.metadata.clone(),
                });
            }
            let sexpr = parse_expr(&mut tokens)?;
            sexprs.push(sexpr);
        }
        Ok(sexprs)
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
                if let Some(next_line) = self.contents.next() {
                    self.curr_line = next_line;
                    self.curr_line_num += 1;
                    self.curr_col_num = 1;
                } else {
                    return None;
                }
                
            }

            // keep track of position on current line
            let pos = self.curr_col_num - 1;
            let c = self.curr_line[pos..].chars().next().unwrap();


            // case for whitespace
            if c.is_whitespace() {
                self.curr_col_num += 1;
                continue;
            } 

            let token_metadata = Metadata {
                line_num: self.curr_line_num,
                col_num: self.curr_col_num,
            };

            // case for left paren
            if c == '(' {
                self.curr_col_num += 1;
                return Some(Token { 
                    lexeme: Lexeme::Lparen,
                    metadata: token_metadata,
                });
            } else if c == ')' {
                self.curr_col_num += 1;
                return Some(Token { 
                    lexeme: Lexeme::Rparen,
                    metadata: token_metadata,
                });

            } else {
                let mut atom = String::new();
                let start_col_num = self.curr_col_num;
  
                while self.curr_col_num <= self.curr_line.len() {
                    let pos = self.curr_col_num -1;
                    let curr_char = self.curr_line[pos..].chars().next().unwrap();
                    if curr_char.is_whitespace() || curr_char == '(' || curr_char == ')' {
                        break;
                    }
                    atom.push(curr_char);
                    self.curr_col_num += 1;
                }

                // returning the atom
                return Some(Token {
                    lexeme: Lexeme::Atom(atom),
                    metadata: Metadata {
                        line_num: self.curr_line_num,
                        col_num: start_col_num,
                    }
                });
            }
        }
    }
}