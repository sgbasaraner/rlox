use crate::RloxError;
use crate::token::{TokenType, Literal, Token, TokenDetails};

pub struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
);

impl Scanner {
    pub fn new(source_code: String) -> Scanner {
        Scanner {
            source_code: source_code,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<RloxError>> {
        let mut errs: Vec<RloxError> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token().map(|e| errs.push(e));
        }

        let eof_token = Token::NonLiteral(
            TokenDetails {
                token_type: TokenType::EOF,
                lexeme: String::new(),
                line: self.line
            }
        );

        self.tokens.push(eof_token);
        if errs.is_empty() { Ok(self.tokens.clone()) } else { Err(errs) }
    }
}

impl Scanner {
    fn source_nth_char(&self, n: usize) -> char {
        self.source_code.chars().nth(n).expect("Scanner error.")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_code.len()
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source_nth_char(self.current - 1)
    }

    fn add_non_literal_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let substring = (self.source_code[self.start..self.current]).to_string();
        let details = TokenDetails {
            token_type: token_type,
            lexeme: substring,
            line: self.line
        };

        let token = match literal {
            Some(v) => Token::Literal(details, v),
            None => Token::NonLiteral(details),
        };

        self.tokens.push(token);
    }

    fn scan_token(&mut self) -> Option<RloxError> {
        let c = self.advance();
        match c {
            '(' => self.add_non_literal_token(TokenType::LeftParen),
            ')' => self.add_non_literal_token(TokenType::RightParen),
            '{' => self.add_non_literal_token(TokenType::LeftBrace),
            '}' => self.add_non_literal_token(TokenType::RightBrace),
            ',' => self.add_non_literal_token(TokenType::Comma),
            '.' => self.add_non_literal_token(TokenType::Dot),
            '-' => self.add_non_literal_token(TokenType::Minus),
            '+' => self.add_non_literal_token(TokenType::Plus),
            ';' => self.add_non_literal_token(TokenType::Semicolon),
            '*' => self.add_non_literal_token(TokenType::Star),
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_non_literal_token(token);
            },
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_non_literal_token(token);
            },
            '<' => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_non_literal_token(token);
            },
            '>' => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_non_literal_token(token);
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_non_literal_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => (), // whitespace, do nothing
            '\n' => self.line = self.line + 1, // newline
            '"' => return self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number()
                } else if is_alphabetic_or_underscore(c) {
                    self.identifier()
                } else {
                    return Some(RloxError::new(self.line, "Unexpected character.", ""));
                }
            }
        };
        None
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source_nth_char(self.current) != expected { 
            return false;
        }

        self.current = self.current + 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source_nth_char(self.current) }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source_code.len() {
            '\0'
        } else {
            self.source_nth_char(self.current + 1)
        }
    }

    fn string(&mut self) -> Option<RloxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1
            }
            self.advance();
        }

        if self.is_at_end() {
            return Some(RloxError::new(self.line, "Unterminated string.", ""));
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes
        let val = (self.source_code[(self.start + 1)..(self.current - 1)]).to_string();
        self.add_token(TokenType::String, Some(Literal::String(val)));
        None
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let float: f64 = self.source_code[self.start..self.current].parse().expect("Float parsing error.");
        self.add_token(TokenType::Number, Some(Literal::Number(float)));
    }

    fn identifier(&mut self) {
        while is_alphanumeric_or_underscore(self.peek()) {
            self.advance();
        }

        let substring = &self.source_code[self.start..self.current];
        let keyword_map = map!{ 
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while"=> TokenType::While
        };
        let token = keyword_map.get(substring).unwrap_or(&TokenType::Identifier);

        match token {
            TokenType::Nil => self.add_token(*token, Some(Literal::Nil)),
            TokenType::True => self.add_token(*token, Some(Literal::True)),
            TokenType::False => self.add_token(*token, Some(Literal::False)),
            _ => self.add_non_literal_token(*token),
        };
    }
}

fn is_alphabetic_or_underscore(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_alphanumeric_or_underscore(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}
