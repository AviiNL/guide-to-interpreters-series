use crate::ast::{
    Program,
    Binary,
    NumericLiteral,
    Identifier, StatementOrExpression, Expression, Statement, VariableDecleration, Assignment, ObjectLiteral, Property, MemberExpr, CallExpr,
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

    fn parse_object_expr(&mut self) -> StatementOrExpression {

        if self.at().t != TokenType::OpenBrace {
            return self.parse_additive_expr();
        }

        self.eat(); // eat the open brace

        let mut properties = Vec::new();

        while !self.is_eof() && self.at().t != TokenType::CloseBrace {

            let key = self.expect(TokenType::Identifier).value;

            // { key, .. }
            if self.at().t == TokenType::Comma {
                self.eat();
                properties.push(Property {
                    key,
                    value: None
                });
                continue;
            }

            // { key }
            if self.at().t == TokenType::CloseBrace {
                properties.push(Property {
                    key,
                    value: None
                });
                continue;
            }
            
            // { key: val, ... }
            self.expect(TokenType::Colon);
            let value = match self.parse_expr() {
                StatementOrExpression::Expression(expr) => expr,
                _ => panic!("Expected expression")
            };

            properties.push(Property {
                key,
                value: Some(Box::new(value))
            });

            if self.at().t != TokenType::CloseBrace {
                self.expect(TokenType::Comma);
            }

        }

        self.expect(TokenType::CloseBrace);
        
        StatementOrExpression::Expression(Expression::ObjectLiteral(
            ObjectLiteral {
                properties
            }
        ))


        //     let key = self.expect(TokenType::Identifier);
        //     self.expect(TokenType::Colon);
        //     let value = match self.parse_expr() {
        //         StatementOrExpression::Expression(expr) => expr,
        //         _ => panic!("Expected expression")
        //     };

        //     properties.push((key.value, value));

        //     if self.at().t == TokenType::Comma {
        //         self.eat();
        //     }
        // }

        // self.expect(TokenType::RBrace);

    }

    fn parse_assignment_expr(&mut self) -> StatementOrExpression {
        let left = self.parse_object_expr();

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
            // self.expect(TokenType::Semicolon);
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
        let mut left = self.parse_call_member_expr();

        while self.at().value == "/" || self.at().value == "*" || self.at().value == "%" {
            let operator = self.eat();
            let right = self.parse_call_member_expr();

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

    // foo.x()
    fn parse_call_member_expr(&mut self) -> StatementOrExpression {
        let member = self.parse_member_expr();

        if self.at().t == TokenType::OpenParen {
            return StatementOrExpression::Expression(self.parse_call_expr(member));
        }

        StatementOrExpression::Expression(*member)
    }

    fn parse_call_expr(&mut self, caller: Box<Expression>) -> Expression {
        let mut call_expr = Expression::Call(
            CallExpr {
                caller,
                arguments: self.parse_args(),
            }
        );

        if self.at().t == TokenType::OpenParen {
            call_expr = self.parse_call_expr(Box::new(call_expr));
        }

        call_expr
    }

    fn parse_args(&mut self) -> Vec<Expression> {
        self.expect(TokenType::OpenParen);

        let args = if self.at().t == TokenType::CloseParen {
            Vec::new()
        } else {
            self.parse_argument_list()
        };

        self.expect(TokenType::CloseParen);

        args
    }

    fn parse_argument_list(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        args.push(self.parse_expr());

        while self.at().t == TokenType::Comma {
            self.eat();
            args.push(self.parse_assignment_expr());
        }

        args.into_iter().map(|arg| match arg {
            StatementOrExpression::Expression(expr) => expr,
            _ => panic!("Expected expression")
        }).collect()
    }

    fn parse_member_expr(&mut self) -> Box<Expression> {
        let mut object = self.parse_primary_expr();

        while self.at().t == TokenType::Dot || self.at().t == TokenType::OpenBracket {
            let operator = self.eat(); // . or [

            let computed = match operator.t {
                TokenType::OpenBracket => true,
                _ => false,
            };

            let property = match operator.t {
                TokenType::Dot => {
                    match self.parse_primary_expr() {
                        StatementOrExpression::Expression(id) => Some(id),
                        _ => panic!("Expected expression")
                    }
                },
                _ => {
                    let p = match self.parse_expr() {
                        StatementOrExpression::Expression(expr) => Some(expr),
                        _ => panic!("Expected expression")
                    };
    
                    self.expect(TokenType::CloseBracket);

                    p
                }
            };

            let extracted_object = match object {
                StatementOrExpression::Expression(expr) => expr,
                _ => panic!("Expected expression")
            };

            object = StatementOrExpression::Expression(Expression::Member(MemberExpr {
                object: Box::new(extracted_object),
                property: Box::new(property.unwrap()),
                computed,
            }));

        }

        match object {
            StatementOrExpression::Expression(expr) => Box::new(expr),
            _ => panic!("Expected expression")
        }
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