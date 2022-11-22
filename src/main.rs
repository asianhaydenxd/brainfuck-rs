#[derive(Eq, PartialEq, Debug)]
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
    Shift(i32),
    Add(i32),
    Loop(Vec<Node>),
    Print,
    Input,
}

struct Interpreter {
    tape: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    fn new() {
        Interpreter {
            tape: Vec::new(),
            pointer: 0
        }
    }

    fn subinterpret(&self, nodes: Vec<Node>) {
        for node in nodes.iter() {
            match node {
                Shift(n) => pointer += n,
                Add(n) => tape[pointer] += n,
                Loop(subnodes) => while tape[pointer] > 0 {
                    self.subinterpret(subnodes);
                }
                Print => {},
                Input => {}
            }
        }
    }
}

fn interpret(nodes: Vec<Node>) {
    let mut interpreter: Interpreter = Interpreter::new();
    interpreter.subinterpret(nodes);
}

fn main() {
    interpret("++++[->+++++<]>.".to_string());
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
