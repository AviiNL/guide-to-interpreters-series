use crate::ast::{
    Program,
    Binary,
    NumericLiteral,
    Identifier, StatementOrExpression, Expression, Statement, VariableDecleration, Assignment,
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
        let current = self.at();
        match current.t {
            TokenType::Let => {
                self.parse_var_decleration()
            }
            TokenType::Const => {
                self.parse_var_decleration()
            }

            _ => self.parse_expr()
        }


        // return self.parse_expr();
    }

    fn parse_var_decleration(&mut self) -> StatementOrExpression {
        let is_const = self.eat().t == TokenType::Const;
        let identifier = self.expect(TokenType::Identifier);
        
        if self.at().t == TokenType::Semicolon {
            self.eat(); // expect semicolon
            if is_const { 
                panic!("Cannot declare a constant without an initial value");
            }

            return StatementOrExpression::Statement(
                Statement::VariableDecleration(
                    VariableDecleration::new(identifier.value, None, is_const)
                )
            );
        }

        self.expect(TokenType::Equals);
        let expr = match self.parse_expr() {
            StatementOrExpression::Expression(expr) => expr,
            _ => panic!("Expected expression")
        };
        self.expect(TokenType::Semicolon);

        StatementOrExpression::Statement(
            Statement::VariableDecleration(
                VariableDecleration::new(identifier.value, Some(expr), is_const)
            )
        )


    }

    fn parse_expr(&mut self) -> StatementOrExpression {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> StatementOrExpression {
        let left = self.parse_additive_expr();

        if self.at().t == TokenType::Equals {
            self.eat(); // advance past equals
            let left = match left {
                StatementOrExpression::Expression(expr) => expr,
                _ => panic!("Expected expression")
            };
            let value = match self.parse_assignment_expr() {
                StatementOrExpression::Expression(expr) => expr,
                _ => panic!("Expected expression")
            };
            self.expect(TokenType::Semicolon);
            return StatementOrExpression::Expression(
                Expression::Assignment(
                    Assignment::new(left, value)
                )
            );
        }

        left
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

            left = StatementOrExpression::Expression(Expression::Binary(Binary {
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

            left = StatementOrExpression::Expression(Expression::Binary(Binary {
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