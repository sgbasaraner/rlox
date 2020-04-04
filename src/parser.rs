use crate::token::{Token, TokenType};
use crate::grammar::Expr;
use crate::RloxError;

#[derive(Debug, Clone)]
pub struct Parser {
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

    pub fn parse(&mut self) -> Result<Expr, RloxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, RloxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, RloxError> {
        self.comparison().and_then(|expr| {
            let mut expr = expr.clone();

            while self.match_toks(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                let clone = self.clone();
                let operator = clone.previous();
                match self.comparison() {
                    Ok(right) => expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)},
                    Err(e) => return Err(e)
                };
            }
    
            Ok(expr)
        })
    }

    fn comparison(&mut self) -> Result<Expr, RloxError> {
        self.addition().and_then(|expr| {
            let mut expr = expr.clone();

            while self.match_toks(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
                let clone = self.clone();
                let operator = clone.previous();
                match self.addition() {
                    Ok(right) => expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)},
                    Err(e) => return Err(e)
                };
            }
    
            Ok(expr)
        })
    }

    fn addition(&mut self) -> Result<Expr, RloxError> {
        self.multiplication().and_then(|expr| {
            let mut expr = expr.clone();

            while self.match_toks(vec![TokenType::Minus, TokenType::Plus]) {
                let clone = self.clone();
                let operator = clone.previous();
                match self.multiplication() {
                    Ok(right) => expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)},
                    Err(e) => return Err(e)
                };
            }
    
            Ok(expr)
        })
    }

    fn multiplication(&mut self) -> Result<Expr, RloxError> {
        self.unary().and_then(|expr| {
            let mut expr = expr.clone();
            while self.match_toks(vec![TokenType::Slash, TokenType::Star]) {
                let clone = self.clone();
                let operator = clone.previous();
                match self.unary() {
                    Ok(right) => expr = Expr::Binary {left: Box::from(expr), operator: operator.clone(), right: Box::from(right)},
                    Err(err) => return Err(err)
                };
            }
    
            return Ok(expr);
        })
    }

    fn unary(&mut self) -> Result<Expr, RloxError> {
        if !self.match_toks(vec![TokenType::Bang, TokenType::Minus]) { 
            return self.primary();
        }
        let clone = self.clone();
        let operator = clone.previous();
        self.unary().and_then(|right| {
            Ok(Expr::Unary { 
                operator: operator.clone(),
                right: Box::from(right)
            })
        })
    }

    fn primary(&mut self) -> Result<Expr, RloxError> {
        let literal_tokens = vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String,
            TokenType::Number
        ];
        if self.match_toks(literal_tokens) { 
            return match self.previous() {
                Token::Literal(_, literal) => Ok(Expr::Literal(literal.clone())),
                _ => Err(RloxError::internal("Parser expected literal.", "")),
            }
        }

        if self.match_toks(vec![TokenType::LeftParen]) {
            return self.expression().and_then(|expr| {
                match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                    Ok(_) => Ok(Expr::Grouping(Box::from(expr))),
                    Err(err) => Err(err)
                }
            });
        }

        Err(err_token(self.peek(), "Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, RloxError> {
        if self.check(&token_type) { 
            Ok(self.advance().clone()) 
        } else {
            Err(err_token(self.peek(), message))
        }
    }
}

fn err_token(token: &Token, message: &str) -> RloxError {
    let details = token.details();
    let location = if details.token_type == TokenType::EOF {
        " at end".to_owned()
    } else {
        format!(" at '{}'!", details.lexeme)
    };
    RloxError::new(details.line, message, &location)
}

impl Parser {
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
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