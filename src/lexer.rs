#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }

            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }

            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }

            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }

            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }

            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }

            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }

            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().ok()?;
                tokens.push(Token::Number(num));
            }

            _ => return None,
        }
    }
    Some(tokens)
}
