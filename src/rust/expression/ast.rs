use std::cmp::min;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyInt, PyString};

use crate::catalog::XCatalog;
use crate::markup::tokens::XNode;

use super::{
    parser::parse_expression,
    tokens::{ExpressionToken, Operator},
};

#[derive(Debug, Clone, IntoPyObject)]
pub enum Literal {
    Int(usize),
    Str(String),
    XNode(XNode),
}

#[derive(Debug, Clone)]
pub enum AST {
    Variable(String),
    Literal(Literal),
    Binary {
        left: Box<AST>,
        op: Operator,
        right: Box<AST>,
    },
}

pub fn parse(tokens: &[ExpressionToken]) -> Option<AST> {
    let mut iter = tokens.iter();
    let tok = iter.next()?;
    let mut left = match tok {
        ExpressionToken::String(s) => AST::Literal(Literal::Str(s.to_string())),
        ExpressionToken::Integer(n) => AST::Literal(Literal::Int(n.clone())),
        ExpressionToken::Ident(ident) => AST::Variable(ident.to_string()),
        ExpressionToken::XNode(n) => AST::Literal(Literal::XNode(n.clone())),
        _ => {
            error!("Left token is ignored: {:?}", tok);
            return None;
        }
    };

    while let Some(op_token) = iter.next() {
        let op = match op_token {
            ExpressionToken::Operator(op) => op.clone(),
            _ => return None,
        };

        let right = match iter.next()? {
            ExpressionToken::String(s) => AST::Literal(Literal::Str(s.to_string())),
            ExpressionToken::Integer(n) => AST::Literal(Literal::Int(n.clone())),
            ExpressionToken::Ident(ident) => AST::Variable(ident.to_string()),
            _ => {
                error!("Right token is ignored: {:?}", tok);
                return None;
            }
        };

        left = AST::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Some(left)
}

use std::collections::HashMap;

pub fn eval_ast<'py>(
    py: Python<'py>,
    ast: &'py AST,
    catalog: &XCatalog,
    params: &HashMap<String, Literal>,
) -> Result<Literal, String> {
    // error!("!!!!!!!!!!!!!!!!!!!!!!!!!!");
    // error!("AST {:?}", ast);

    match ast {
        AST::Literal(lit) => Ok(lit.clone()),

        AST::Binary { left, op, right } => {
            let l = eval_ast(py, left, catalog, params)?;
            let r = eval_ast(py, right, catalog, params)?;

            match (l, r, op) {
                (Literal::Int(a), Literal::Int(b), Operator::Add) => Ok(Literal::Int(a + b)),
                (Literal::Int(a), Literal::Int(b), Operator::Sub) => Ok(Literal::Int(a - b)),
                (Literal::Int(a), Literal::Int(b), Operator::Mul) => Ok(Literal::Int(a * b)),
                (Literal::Int(a), Literal::Int(b), Operator::Div) => {
                    if b == 0 {
                        Err("division by zero".to_string())
                    } else {
                        Ok(Literal::Int(a / b))
                    }
                }

                (Literal::Str(a), Literal::Str(b), Operator::Add) => Ok(Literal::Str(a + &b)),

                _ => Err("unsupported operand types".to_string()),
            }
        }

        AST::Variable(name) => match params.get(name) {
            Some(Literal::Int(i)) => Ok(Literal::Int(i.clone())),
            Some(Literal::Str(s)) => Ok(Literal::Str(s.clone())),
            Some(Literal::XNode(node)) => {
                let resp = catalog.render_node(py, node, PyDict::new(py));
                resp.map(|markup| Literal::Str(markup))
                    .map_err(|err| format!("Cant render {}: {}", node, err))
            }
            None => Err(format!("Undefined variable: {}", name)),
        },
    }
}

fn cast_params<'py>(params: Bound<'py, PyDict>) -> Result<HashMap<String, Literal>, PyErr> {
    let mut result = HashMap::new();

    for (key, value) in params.iter() {
        let key_str = key.downcast::<PyString>()?.to_string();
        if let Ok(val_str) = value.downcast::<PyString>() {
            result.insert(key_str, Literal::Str(val_str.to_string()));
        } else if let Ok(val_int) = value.downcast::<PyInt>() {
            result.insert(key_str, Literal::Int(val_int.extract::<usize>()?));
        } else if let Ok(val_xnode) = value.extract::<XNode>() {
            result.insert(key_str, Literal::XNode(val_xnode));
        } else {
            let err: PyErr = PyTypeError::new_err(format!("Can't parse parameter {:?}", value));
            return Err(err);
        }
    }

    Ok(result)
}

pub fn eval_expression<'py>(
    py: Python<'py>,
    expression: &str,
    catalog: &XCatalog,
    params: Bound<'py, PyDict>,
) -> Result<Literal, String> {
    info!("Evaluating expression {}...", &expression[..min(expression.len(), 24)]);
    debug!("{}", expression);
    let tokens = parse_expression(expression)?;
    let params_ast = cast_params(params).map_err(|e| format!("{}", e))?;
    let ast = parse(tokens.as_slice()).unwrap();
    eval_ast(py, &ast, catalog, &params_ast)
}
