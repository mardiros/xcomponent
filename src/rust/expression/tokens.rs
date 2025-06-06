use std::{collections::HashMap, fmt, str::FromStr};

use crate::markup::tokens::XNode;
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum ExpType {
    Expression,
    Ident,
    Operator,
    String,
    Integer,
    Boolean,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OperatorErr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
}

impl FromStr for UnaryOperator {
    type Err = OperatorErr;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "not" => Ok(UnaryOperator::Not),
            _ => Err(OperatorErr),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            UnaryOperator::Not => "not",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl FromStr for Operator {
    type Err = OperatorErr;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            "and" => Ok(Operator::And),
            "or" => Ok(Operator::Or),
            "==" => Ok(Operator::Eq),
            "!=" => Ok(Operator::Neq),
            ">" => Ok(Operator::Gt),
            "<" => Ok(Operator::Lt),
            ">=" => Ok(Operator::Gte),
            "<=" => Ok(Operator::Lte),
            _ => Err(OperatorErr),
        }
    }
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Or => 1,
            Operator::And => 2,
            Operator::Eq | Operator::Neq => 3,
            Operator::Gt | Operator::Gte | Operator::Lt | Operator::Lte => 4,
            Operator::Add | Operator::Sub => 5,
            Operator::Mul | Operator::Div => 6,
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
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Eq => "==",
            Operator::Neq => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOp {
    Field(String),
    Index(Box<ExpressionToken>),
    Call {
        args: Vec<ExpressionToken>,
        kwargs: HashMap<String, ExpressionToken>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    BinaryExpression(Vec<ExpressionToken>),
    UnaryExpression {
        op: UnaryOperator,
        expr: Box<ExpressionToken>,
    },
    Ident(String),
    Operator(Operator),
    String(String),
    // Uuid(String),
    Integer(isize),
    Boolean(bool),
    XNode(XNode),
    PostfixOp(PostfixOp),
    IfExpression {
        condition: Box<ExpressionToken>,
        then_branch: Box<ExpressionToken>,
        else_branch: Option<Box<ExpressionToken>>,
    },
    ForExpression {
        ident: String,
        iterable: Box<ExpressionToken>,
        body: Box<ExpressionToken>,
    },
    LetExpression {
        ident: String,
        expr: Box<ExpressionToken>,
    },
    Noop,
}

impl std::fmt::Display for ExpressionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionToken::BinaryExpression(children) => {
                write!(
                    f,
                    "{}",
                    children.iter().map(|v| v.to_string()).collect::<String>()
                )
            }
            ExpressionToken::UnaryExpression { op, expr } => {
                write!(f, "{} {}", op, expr)
            }
            ExpressionToken::Ident(ident) => {
                write!(f, "{}", ident)
            }
            ExpressionToken::Operator(op) => write!(f, " {} ", op.to_string()),
            ExpressionToken::String(value) => {
                write!(f, "\"{}\"", value.replace('"', "\\\""))
            }
            // ExpressionToken::Uuid(value) => write!(f, "\"{}\"", value),
            ExpressionToken::Integer(value) => write!(f, "{}", value),
            ExpressionToken::Boolean(value) => write!(f, "{}", value),
            ExpressionToken::XNode(n) => write!(f, "{}", n),
            ExpressionToken::PostfixOp(op) => match op {
                PostfixOp::Field(field) => write!(f, ".{}", field),
                PostfixOp::Index(index) => write!(f, "[{}]", index),
                // FIXME, display the args and kwargs properly
                PostfixOp::Call { args, kwargs } => write!(f, "({:?}, {:?})", args, kwargs),
            },
            ExpressionToken::IfExpression {
                condition,
                then_branch,
                else_branch,
            } => match else_branch {
                None => write!(f, "if {} {{ {} }}", condition, then_branch),
                Some(else_branch) => {
                    write!(
                        f,
                        "if {} {{ {} }} else {{ {} }}",
                        condition, then_branch, else_branch
                    )
                }
            },
            ExpressionToken::ForExpression {
                ident,
                iterable,
                body,
            } => write!(f, "for {} in {} {{ {} }}", ident, iterable, body),
            ExpressionToken::LetExpression { ident, expr } => write!(f, "let {} = {}", ident, expr),
            ExpressionToken::Noop => write!(f, ""), // ??
        }
    }
}
