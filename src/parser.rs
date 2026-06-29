use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Binary {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            return Some(token);
        }
        None
    }

    fn get_precedence(op: &Token) -> i32 {
        match op {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            _ => 0,
        }
    }

    pub fn parse(&mut self, min_precedence: i32) -> Option<Expr> {
        let token = self.next_token()?;

        let mut left = match token {
            Token::Number(n) => Expr::Number(n),
            Token::Minus => {
                let right = self.parse(3)?;
                Expr::Unary {
                    op: Token::Minus,
                    expr: Box::new(right),
                }
            }
            Token::Plus => self.parse(3)?,
            Token::LParen => {
                let inner_expr = self.parse(0)?;
                match self.next_token() {
                    Some(Token::RParen) => inner_expr,
                    _ => return None,
                }
            }
            _ => return None,
        };

        loop {
            let next_op = match self.peek() {
                Some(t) if Self::get_precedence(t) > min_precedence => t.clone(),
                _ => break,
            };

            self.next_token();
            let op_precendence = Self::get_precedence(&next_op);
            let right = self.parse(op_precendence)?;

            left = Expr::Binary {
                op: next_op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        Some(left)
    }
}
