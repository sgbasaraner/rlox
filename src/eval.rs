use crate::grammar::Expr;
use crate::token::{Literal, TokenType};
use crate::RloxError;

pub trait Evaluable {
    fn evaluate(&self) -> Result<Literal, RloxError>;
}

impl Evaluable for Expr {
    fn evaluate(&self) -> Result<Literal, RloxError> {
        match self {
            Expr::Binary { left, operator, right } => {
                left.evaluate().and_then(|left| right.evaluate().and_then(|right| {
                    match operator.details().token_type {
                        TokenType::Greater => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::from(l > r))),
                        TokenType::GreaterEqual => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::from(l >= r))),
                        TokenType::Less => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::from(l < r))),
                        TokenType::LessEqual => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::from(l <= r))),
                        TokenType::Minus => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::Number(l - r))),
                        TokenType::Slash => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::Number(l / r))),
                        TokenType::Star => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::Number(l * r))),
                        TokenType::Plus => cast_nums(&left, &right).and_then(|(l, r)| Ok(Literal::Number(l + r)))
                            .or_else(|_| cast_strs(&left, &right).and_then(|(l, r)| Ok(Literal::String(format!("{}{}", l, r)))))
                            .or(Err(cast_err("string or number"))),
                        TokenType::EqualEqual => Ok(Literal::from(left.is_equal(&right))),
                        TokenType::BangEqual => Ok(Literal::from(!left.is_equal(&right))),
                        _ => Ok(Literal::Nil) // unreachable
                    }
                }))
            },
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Unary { operator, right } => {
                right.evaluate().and_then(|right| {
                    match operator.details().token_type {
                        TokenType::Bang => Ok(Literal::from(!right.is_truthy())),
                        TokenType::Minus => right.cast_number().and_then(|n| Ok(Literal::Number(-n))),
                        _ => Ok(Literal::Nil) // unreachable
                    }
                })
            }
        }
    }
}

fn cast_nums(left: &Literal, right: &Literal) -> Result<(f64, f64), RloxError> {
    left.cast_number().and_then(|left| right.cast_number().and_then(|right| Ok((left, right))))
}

fn cast_strs(left: &Literal, right: &Literal) -> Result<(String, String), RloxError> {
    left.cast_string().and_then(|left| right.cast_string().and_then(|right| Ok((left, right))))
}

impl Literal {
    fn cast_number(&self) -> Result<f64, RloxError> {
        match self {
            Literal::Number(n) => Ok(*n),
            _ => Err(cast_err("number")),
        }
    }

    fn cast_string(&self) -> Result<String, RloxError> {
        match self {
            Literal::String(s) => Ok(s.clone()),
            _ => Err(cast_err("string")),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Literal::Nil | Literal::False => false,
            _ => true
        }
    }

    fn from(boolean: bool) -> Literal {
        if boolean { Literal::True } else { Literal::False }
    }

    fn is_equal(&self, other: &Literal) -> bool {
        match self {
            Literal::Nil => {
                match other {
                    Literal::Nil => true,
                    _ => false
                }
            },
            _ => self == other
        }
    }
}

fn cast_err(cast_type: &str) -> RloxError {
    RloxError::internal(&format!("Couldn't parse {}", cast_type), "")
}