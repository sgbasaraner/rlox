use std::fmt;

// TODO: this type could be split into two, one with literal and one without
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: i32
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Nil
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

#[derive(Debug, Clone, Copy)]
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