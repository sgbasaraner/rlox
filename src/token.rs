use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    NonLiteral(TokenDetails),
    Literal(TokenDetails, Literal)
}

impl Token {
    pub fn details(&self) -> TokenDetails {
        match self {
            Token::NonLiteral(details) => details.clone(),
            Token::Literal(details, _) => details.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenDetails {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Nil,
    True,
    False
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Literal::String(string) => string.to_owned(),
            Literal::Number(n) => {
                let num = n;
                format!("{}", num)
            },
            Literal::Nil => "nil".to_owned()
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.           
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,           

    // One or two character tokens.     
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual, 

    // Literals.                                     
    Identifier, String, Number,

    // Keywords.                                     
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,    

    EOF
}