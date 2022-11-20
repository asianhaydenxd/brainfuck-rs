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

fn parse(code: String) -> Vec<Token> {
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

fn interpret(code: String) {
    println!("{:?}", parse(code));
    println!("Interpreter complete");
}

fn main() {
    interpret("++++[->+++++<]>.".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(parse("<>+-[].,".to_string()), vec![
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
