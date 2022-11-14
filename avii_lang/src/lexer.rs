#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Literal Types
    Number,
    String,
    Identifier,
    
    // Keywords
    Let,
    Const,
    Func,
    
    // Grouping * Operators
    BinaryOperator,
    Equals,
    OpenParen,
    CloseParen,
    
    // Delimiters
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;

    Dot, // .

    OpenBrace,  // {
    CloseBrace, // }

    OpenBracket,  // [
    CloseBracket, // ]

    // End of File
    EOF,
}

static KEYWORDS: &[(&str, TokenType)] = &[
    ("let", TokenType::Let),
    ("const", TokenType::Const),
    ("func", TokenType::Func),
];

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub t: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(value: String, t: TokenType, line: usize) -> Token {
        Token { value, t, line }
    }
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();

    let mut line = 1;

    // Build each token util end of file
    while let Some(c) = chars.next() {
        match c {
            '\n' => line += 1,
            '(' => tokens.push(Token::new("(".to_string(), TokenType::OpenParen, line)),
            ')' => tokens.push(Token::new(")".to_string(), TokenType::CloseParen, line)),
            '{' => tokens.push(Token::new("{".to_string(), TokenType::OpenBrace, line)),
            '}' => tokens.push(Token::new("}".to_string(), TokenType::CloseBrace, line)),
            '[' => tokens.push(Token::new("[".to_string(), TokenType::OpenBracket, line)),
            ']' => tokens.push(Token::new("]".to_string(), TokenType::CloseBracket, line)),
            '+' => tokens.push(Token::new("+".to_string(), TokenType::BinaryOperator, line)),
            '-' => tokens.push(Token::new("-".to_string(), TokenType::BinaryOperator, line)),
            '*' => tokens.push(Token::new("*".to_string(), TokenType::BinaryOperator, line)),
            '/' => tokens.push(Token::new("/".to_string(), TokenType::BinaryOperator, line)),
            '%' => tokens.push(Token::new("%".to_string(), TokenType::BinaryOperator, line)),
            '=' => tokens.push(Token::new("=".to_string(), TokenType::Equals, line)),
            ';' => tokens.push(Token::new(";".to_string(), TokenType::Semicolon, line)),
            ':' => tokens.push(Token::new(":".to_string(), TokenType::Colon, line)),
            ',' => tokens.push(Token::new(",".to_string(), TokenType::Comma, line)),
            '.' => tokens.push(Token::new(".".to_string(), TokenType::Dot, line)),
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {

                    if c == '\\' {
                        // escape characters
                        if let Some(c) = chars.next() {
                            match c {
                                'n' => string.push('\n'),
                                'r' => string.push('\r'),
                                't' => string.push('\t'),
                                '0' => string.push('\0'),
                                '"' => string.push('"'),
                                '\'' => string.push('\''),
                                '\\' => string.push('\\'),
                                _ => panic!("Invalid escape character on line {}", line),
                            }
                        }
                        continue;
                    }

                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                tokens.push(Token::new(string, TokenType::String, line));
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);
                while let Some(&('0'..='9')) = chars.peek() {
                    number.push(chars.next().unwrap());
                }

                if chars.peek() == Some(&'.') {
                    number.push(chars.next().unwrap());
                    if let Some(&('0'..='9')) = chars.peek() {
                        while let Some(&('0'..='9')) = chars.peek() {
                            number.push(chars.next().unwrap());
                        }
                    } else {
                        panic!("Invalid number on line {}", line);
                    }
                }

                tokens.push(Token::new(number, TokenType::Number, line));
            }
            'A'..='Z' | 'a'..='z' | '_' => { // allow uppercase, lowercase, and underscore as a starting character
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&('A'..='Z' | 'a'..='z' | '_' | '0'..='9')) = chars.peek() { // also allow numbers but not as starting characer
                    identifier.push(chars.next().unwrap());
                }

                // check for reserved keywords
                let token_type = KEYWORDS
                    .iter()
                    .find(|(keyword, _)| *keyword == identifier)
                    .map(|(_, token_type)| *token_type)
                    .unwrap_or(TokenType::Identifier);

                tokens.push(Token::new(identifier, token_type, line));
            },
            _ => {
                if c.is_whitespace() {
                    continue;
                }

                panic!("Unhandled character \"{}\" on line {}", c, line);
            },
        }
    }

    tokens.push(Token::new("EndOfFile".to_string(), TokenType::EOF, line));

    return tokens;
}