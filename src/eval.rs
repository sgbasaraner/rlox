use crate::grammar::Expr;
use crate::token::{Literal, TokenType, Token};
use crate::RloxError;
use std::fmt;

pub trait Evaluable {
    fn evaluate(&self) -> Result<Value, RloxError>;
}

impl Evaluable for Expr {
    fn evaluate(&self) -> Result<Value, RloxError> {
        match self {
            Expr::Binary { left, operator, right } => eval_binary(left, operator, right),
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(literal) => Ok(Value::from(literal)),
            Expr::Unary { operator, right } => eval_unary(operator, right)
        }
    }
}

fn eval_unary(operator: &Token, right: &Box<Expr>) -> Result<Value, RloxError> {
    right.evaluate().and_then(|right| {
        match operator.details().token_type {
            TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
            TokenType::Minus => right.cast_number().and_then(|n| Ok(Value::Number(-n))),
            _ => Ok(Value::Nil) // unreachable
        }
    })
}

fn eval_binary(left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Result<Value, RloxError> {
    left.evaluate().and_then(|left| right.evaluate().and_then(|right| {
        match operator.details().token_type {
            TokenType::Greater => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Boolean(l > r))),
            TokenType::GreaterEqual => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Boolean(l >= r))),
            TokenType::Less => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Boolean(l < r))),
            TokenType::LessEqual => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Boolean(l <= r))),
            TokenType::Minus => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Number(l - r))),
            TokenType::Slash => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Number(l / r))),
            TokenType::Star => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Number(l * r))),
            TokenType::Plus => cast_nums(&left, &right).and_then(|(l, r)| Ok(Value::Number(l + r)))
                .or_else(|_| cast_strs(&left, &right).and_then(|(l, r)| Ok(Value::String(format!("{}{}", l, r)))))
                .or(Err(cast_err("string or number"))),
            TokenType::EqualEqual => Ok(Value::Boolean(left.is_equal(&right))),
            TokenType::BangEqual => Ok(Value::Boolean(!left.is_equal(&right))),
            _ => Ok(Value::Nil) // unreachable
        }
    }))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

impl Value {
    fn from(literal: &Literal) -> Value {
        match literal {
            Literal::String(string) => Value::String(string.clone()),
            Literal::Number(n) => Value::Number(*n),
            Literal::Nil => Value::Nil,
            Literal::True => Value::Boolean(true),
            Literal::False => Value::Boolean(false)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Value::String(string) => string.to_owned(),
            Value::Number(n) => {
                let num = n;
                format!("{}", num)
            },
            Value::Nil => "nil".to_owned(),
            Value::Boolean(b) => String::from(if *b { "true" } else { "false" })
        };
        write!(f, "{}", string)
    }
}

fn cast_nums(left: &Value, right: &Value) -> Result<(f64, f64), RloxError> {
    left.cast_number().and_then(|left| right.cast_number().and_then(|right| Ok((left, right))))
}

fn cast_strs(left: &Value, right: &Value) -> Result<(String, String), RloxError> {
    left.cast_string().and_then(|left| right.cast_string().and_then(|right| Ok((left, right))))
}

impl Value {
    fn cast_number(&self) -> Result<f64, RloxError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(cast_err("number")),
        }
    }

    fn cast_string(&self) -> Result<String, RloxError> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err(cast_err("string")),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true
        }
    }

    fn is_equal(&self, other: &Value) -> bool {
        match self {
            Value::Nil => {
                match other {
                    Value::Nil => true,
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