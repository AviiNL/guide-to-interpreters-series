use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,
    Let,
    EOF,
}

static KEYWORDS: &[(&str, TokenType)] = &[
    ("let", TokenType::Let),
];

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub t: TokenType,
}

impl Token {
    pub fn new(value: String, t: TokenType) -> Token {
        Token { value, t }
    }
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();

    // Build each token util end of file
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::new("(".to_string(), TokenType::OpenParen)),
            ')' => tokens.push(Token::new(")".to_string(), TokenType::CloseParen)),
            '+' => tokens.push(Token::new("+".to_string(), TokenType::BinaryOperator)),
            '-' => tokens.push(Token::new("-".to_string(), TokenType::BinaryOperator)),
            '*' => tokens.push(Token::new("*".to_string(), TokenType::BinaryOperator)),
            '/' => tokens.push(Token::new("/".to_string(), TokenType::BinaryOperator)),
            '%' => tokens.push(Token::new("%".to_string(), TokenType::BinaryOperator)),
            '=' => tokens.push(Token::new("=".to_string(), TokenType::Equals)),
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);
                while let Some(&('0'..='9')) = chars.peek() {
                    number.push(chars.next().unwrap());
                }
                tokens.push(Token::new(number, TokenType::Number));
            }
            'A'..='z' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&('A'..='z')) = chars.peek() {
                    identifier.push(chars.next().unwrap());
                }

                // check for reserved keywords
                let token_type = KEYWORDS
                    .iter()
                    .find(|(keyword, _)| *keyword == identifier)
                    .map(|(_, token_type)| *token_type)
                    .unwrap_or(TokenType::Identifier);

                tokens.push(Token::new(identifier, token_type));
            },
            _ => {
                if c.is_whitespace() {
                    continue;
                }

                panic!("Unhandled character \"{}\"", c);
            },
        }
    }

    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF));

    return tokens;
}