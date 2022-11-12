use crate::ast::{
    Stmt,
    Program,
    Expr,
    BinaryExpr,
    NumericLiteral,
    Identifier, StatementOrExpression, Expression, Statement, Let,
};

use crate::lexer::{tokenize, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn produce_ast(source_code: &str) -> Program {
        let tokens = tokenize(source_code);
        let mut parser = Parser { tokens };
        let mut body = Vec::new();

        while !parser.is_eof() {
            let stmt = parser.parse_stmt();
            body.push(stmt);
        }

        Program { body }
    }

    fn at(&self) -> &Token {
        &self.tokens[0]
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, t: TokenType) -> Token {
        let token = self.eat();
        if token.t != t {
            panic!("Expected {:?}, got {:?}", t, token.t);
        }
        token
    }

    fn is_eof(&self) -> bool {
        // if the first token is of type EOF, then we are at the end of the file
        self.tokens[0].t == TokenType::EOF
    }

    fn parse_stmt(&mut self) -> StatementOrExpression {
        return self.parse_expr();
    }

    fn parse_expr(&mut self) -> StatementOrExpression {
        return self.parse_additive_expr();
    }

    fn parse_additive_expr(&mut self) -> StatementOrExpression {
        let mut left = self.parse_multiplicitive_expr();

        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat();
            let right = self.parse_multiplicitive_expr();

            let extracted_left = match left {
                StatementOrExpression::Expression(e) => e,
                _ => panic!("Expected expression"),
            };

            let extracted_right = match right {
                StatementOrExpression::Expression(e) => e,
                _ => panic!("Expected expression"),
            };

            left = StatementOrExpression::Expression(Expression::BinaryExpr(BinaryExpr {
                left: Box::new(extracted_left),
                operator: operator.value,
                right: Box::new(extracted_right),
            }));
        }

        left
    }

    fn parse_multiplicitive_expr(&mut self) -> StatementOrExpression {
        let mut left = self.parse_primary_expr();

        while self.at().value == "/" || self.at().value == "*" || self.at().value == "%" {
            let operator = self.eat();
            let right = self.parse_primary_expr();

            let extracted_left = match left {
                StatementOrExpression::Expression(e) => e,
                _ => panic!("Expected expression"),
            };

            let extracted_right = match right {
                StatementOrExpression::Expression(e) => e,
                _ => panic!("Expected expression"),
            };

            left = StatementOrExpression::Expression(Expression::BinaryExpr(BinaryExpr {
                left: Box::new(extracted_left),
                operator: operator.value,
                right: Box::new(extracted_right),
            }));
        }

        left
    }

    fn parse_primary_expr(&mut self) -> StatementOrExpression {
        let tk = self.at().t.clone();

        match tk {
            TokenType::Number => {
                let token = self.eat();
                let value = token.value.parse::<f64>().unwrap();
                return StatementOrExpression::Expression(Expression::NumericLiteral( NumericLiteral { value }));
            }
            TokenType::Identifier => {
                let token = self.eat();
                let symbol = token.value;
                return StatementOrExpression::Expression(Expression::Identifier( Identifier { symbol }));
            }
            TokenType::OpenParen => {
                self.eat();
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                return expr;
            }
            _ => panic!("Unexpected token: {:?}", self.at()),
        }
    }
}