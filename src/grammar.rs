use crate::token::{Token, Literal}; 
use std::fmt;

#[derive(Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary { operator: Token, right: Box<Expr> }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Expr::Binary { left, operator, right } => parenthesize(&operator.details().lexeme, vec![left, right]),
            Expr::Grouping(expr) => parenthesize("group", vec![expr]),
            Expr::Literal(literal) => format!("{}", literal),
            Expr::Unary { operator, right } => parenthesize(&operator.details().lexeme, vec![right])
        };
        write!(f, "{}", string)
    }
}

fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let mut string = String::from("(");
    string.push_str(name);
    for expr in exprs {
        string.push_str(&format!(" {}", expr));
    }
    string.push_str(")");
    string
}