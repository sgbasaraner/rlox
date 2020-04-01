use crate::token::{Token, TokenType};
use crate::grammar::Expr;

#[derive(Debug, Clone)]
struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0
        }
    }
}

impl Parser {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_toks(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let clone = self.clone();
            let operator = clone.previous();
            let right = self.comparison();
            expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)};
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        while self.match_toks(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let clone = self.clone();
            let operator = clone.previous();
            let right = self.addition();
            expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)};
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while self.match_toks(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let clone = self.clone();
            let operator = clone.previous();
            let right = self.multiplication();
            expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)};
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_toks(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let clone = self.clone();
            let operator = clone.previous();
            let right = self.unary();
            expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)};
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if !self.match_toks(vec![TokenType::Bang, TokenType::Minus]) { 
            return self.primary();
        }
        let clone = self.clone();
        let operator = clone.previous();
        let right = self.unary();
        Expr::Unary { 
            operator: operator.clone(),
            right: Box::from(right)
        }
    }

    fn primary(&mut self) -> Expr {
        let literal_tokens = vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String,
            TokenType::Number
        ];
        if self.match_toks(literal_tokens) { 
            match self.previous() {
                Token::Literal(_, literal) => return Expr::Literal(literal.clone()),
                _ => panic!("Parser error. Expected literal."),
            }
        }

        if self.match_toks(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::from(expr));
        }

        panic!("Parser error.")
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        unimplemented!()
    }
}

impl Parser {
    fn advance(&mut self) -> &Token {
        if (!self.is_at_end()) {
            self.current = self.current + 1;
        }
        self.previous()
    }

    fn check(&self, token: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().details().token_type == *token
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().details().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn match_toks(&mut self, types: Vec<TokenType>) -> bool {
        for tok in types {
            if !self.check(&tok) { continue; }
            self.advance();
            return true;
        }

        false
    }      
}