#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Token {
    Left,
    Right,
    Plus,
    Minus,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
}

fn lex(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for character in code.chars() {
        match character {
            '<' => tokens.push(Token::Left),
            '>' => tokens.push(Token::Right),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            _ => {},
        };
    }

    tokens
}

enum Node {
    Shift(isize),
    Add(isize),
    Loop(Vec<Node>),
    Print,
    Input,
}

struct Parser {
    index: usize,
    tokens: Vec<Token>,
    loop_descent: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            index: 0,
            tokens,
            loop_descent: 0,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current_token(&self) -> Token {
        self.tokens[self.index]
    }

    fn is_valid(&self) -> bool {
        self.index < self.tokens.len()
    }
    
    fn parse_shift_sequence(&mut self) -> Node {
        let mut value: isize = 0;
        while self.is_valid() {
            match self.current_token() {
                Token::Left => value -= 1,
                Token::Right => value += 1,
                _ => break,
            }
            self.advance();
        }
        Node::Shift(value)
    }

    fn parse_add_sequence(&mut self) -> Node {
        let mut value: isize = 0;
        while self.is_valid() {
            match self.current_token() {
                Token::Minus => value -= 1,
                Token::Plus => value += 1,
                _ => break,
            }
            self.advance();
        }
        Node::Add(value)
    }

    fn subparse(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        while self.is_valid() {
            match self.current_token() {
                Token::Left | Token::Right => {
                    nodes.push(self.parse_shift_sequence());
                },
                Token::Plus | Token::Minus => {
                    nodes.push(self.parse_add_sequence());
                },
                Token::LeftBracket => {
                    self.advance();
                    self.loop_descent += 1;
                    nodes.push(Node::Loop(self.subparse()));
                },
                Token::RightBracket => {
                    self.advance();
                    if self.loop_descent == 0 {
                        panic!("Unmatched right bracket");
                    }
                    self.loop_descent -= 1;
                    return nodes;
                },
                Token::Dot => {
                    nodes.push(Node::Print);
                    self.advance();
                },
                Token::Comma => {
                    nodes.push(Node::Input);
                    self.advance()
                },
            }   
        }
        if self.loop_descent > 0 {
            panic!("Unmatched left bracket");
        }
        nodes
    }
}

fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut parser: Parser = Parser::new(tokens);
    parser.subparse()
}

struct Interpreter {
    tape: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            tape: vec![0],
            pointer: 0
        }
    }

    fn subinterpret(&mut self, nodes: &Vec<Node>) {
        for node in nodes.iter() {
            match node {
                Node::Shift(n) => if *n > 0 {
                    self.pointer += *n as usize;
                    if self.pointer == self.tape.len() {
                        self.tape.push(0);
                    }
                } else {
                    self.pointer -= -*n as usize;
                },
                Node::Add(n) => {
                    let original = self.tape[self.pointer] as isize;
                    self.tape[self.pointer] = ((original + n) % 256) as u8
                },
                Node::Loop(subnodes) => {
                    while self.tape[self.pointer] > 0 {
                        self.subinterpret(&subnodes);
                    }
                }
                Node::Print => println!("{}", self.tape[self.pointer]),
                Node::Input => {}
            }
        }
    }
}

fn interpret(nodes: Vec<Node>) {
    let mut interpreter: Interpreter = Interpreter::new();
    interpreter.subinterpret(&nodes);
}

fn main() {
    interpret(parse(lex("++++[->++++<]>.".to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(lex("<>+-[].,".to_string()), vec![
            Token::Left,
            Token::Right,
            Token::Plus,
            Token::Minus,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Dot,
            Token::Comma,
        ]);
    }
}
