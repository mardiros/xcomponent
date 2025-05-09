use std::{fmt, str::FromStr};

use pyo3::prelude::*;

use crate::markup::tokens::XNode;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum ExpType {
    Expression,
    Ident,
    Operator,
    String,
    Integer,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OperatorErr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = OperatorErr;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(OperatorErr),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    Expression(Vec<ExpressionToken>),
    Ident(String),
    Operator(Operator),
    String(String),
    Integer(usize),
    XNode(XNode),
}

impl std::fmt::Display for ExpressionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionToken::Expression(children) => {
                write!(
                    f,
                    "{}",
                    children.iter().map(|v| v.to_string()).collect::<String>()
                )
            }
            ExpressionToken::Ident(ident) => {
                write!(f, "{}", ident)
            }
            ExpressionToken::Operator(op) => write!(f, " {} ", op.to_string()),
            ExpressionToken::String(value) => {
                write!(f, "\"{}\"", value.replace('"', "\\\""))
            }
            ExpressionToken::Integer(value) => write!(f, "{}", value),
            ExpressionToken::XNode(n) => write!(f, "{}", n),
        }
    }
}
