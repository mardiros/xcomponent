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
    Boolean,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OperatorErr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
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
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Eq => "==",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    ident: String,
    params: Vec<ExpressionToken>,
}

impl FunctionCall {
    pub fn new(ident: String, params: Vec<ExpressionToken>) -> Self {
        FunctionCall { ident, params }
    }
    pub fn ident(&self) -> &str {
        return self.ident.as_str();
    }
    pub fn params(&self) -> &Vec<ExpressionToken> {
        return self.params.as_ref();
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = (self
            .params
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>())
        .join(",");
        write!(f, "{}({})", self.ident, params)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionToken {
    BinaryExpression(Vec<ExpressionToken>),
    Ident(String),
    Operator(Operator),
    String(String),
    Integer(isize),
    Boolean(bool),
    XNode(XNode),
    FuncCall(FunctionCall),
    IfExpression {
        condition: Box<ExpressionToken>,
        then_branch: Box<ExpressionToken>,
        else_branch: Option<Box<ExpressionToken>>,
    },
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
            ExpressionToken::Ident(ident) => {
                write!(f, "{}", ident)
            }
            ExpressionToken::Operator(op) => write!(f, " {} ", op.to_string()),
            ExpressionToken::String(value) => {
                write!(f, "\"{}\"", value.replace('"', "\\\""))
            }
            ExpressionToken::Integer(value) => write!(f, "{}", value),
            ExpressionToken::Boolean(value) => write!(f, "{}", value),
            ExpressionToken::XNode(n) => write!(f, "{}", n),
            ExpressionToken::FuncCall(func) => write!(f, "{}", func),
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
        }
    }
}
