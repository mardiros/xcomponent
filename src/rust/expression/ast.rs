use std::cmp::min;
use std::collections::HashMap;

use pyo3::exceptions::{PySyntaxError, PyTypeError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyInt, PyString, PyTuple};

use crate::catalog::XCatalog;
use crate::markup::tokens::XNode;

use super::{
    parser::parse_expression,
    tokens::{ExpressionToken, Operator},
};

trait Truthy {
    fn is_truthy(&self) -> bool;
}

#[derive(Debug, Clone, IntoPyObject)]
pub enum Literal {
    Bool(bool),
    Int(isize),
    Str(String),
    XNode(XNode),
}

impl Literal {
    fn downcast<'py>(value: Bound<'py, PyAny>) -> Result<Self, PyErr> {
        if let Ok(v) = value.downcast::<PyString>() {
            return Ok(Literal::Str(v.to_string()));
        } else if let Ok(v) = value.downcast::<PyBool>() {
            return Ok(Literal::Bool(v.extract::<bool>()?));
        } else if let Ok(v) = value.downcast::<PyInt>() {
            return Ok(Literal::Int(v.extract::<isize>()?));
        } else if let Ok(v) = value.extract::<XNode>() {
            return Ok(Literal::XNode(v));
        } else {
            let err: PyErr = PyTypeError::new_err(format!("Can't parse parameter {:?}", value));
            return Err(err);
        }
    }
}

impl Truthy for Literal {
    fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(bool) => bool.clone(),
            Literal::Int(i) => *i != 0,
            Literal::Str(s) => !s.is_empty(),
            Literal::XNode(_) => true,
        }
    }
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
    FuncCall {
        name: String,
        args: Vec<AST>,
    },
    IfStatement {
        condition: Box<AST>,
        then_branch: Box<AST>,
        else_branch: Option<Box<AST>>,
    },
}

fn token_to_ast(tok: &ExpressionToken) -> Result<AST, PyErr> {
    let ast = match tok {
        ExpressionToken::BinaryExpression(ex) => match ex.len() {
            0 => Err(PySyntaxError::new_err(format!(
                "Syntax error, expected statement"
            ))),
            1 => token_to_ast(ex.first().unwrap()),
            2 => Err(PySyntaxError::new_err(format!("Syntax error near {}", tok))),
            _ => {
                let mut iter = ex.into_iter();
                let leftwrapper = iter.next().unwrap();
                let left = Box::new(token_to_ast(leftwrapper)?);
                let opwrap = iter.next().unwrap();
                let op = match opwrap {
                    ExpressionToken::Operator(op) => op.clone(),
                    _ => {
                        return Err(PySyntaxError::new_err(format!(
                            "Syntax error, operator expected not {}",
                            opwrap
                        )))
                    }
                };
                let right = Box::new(parse(iter.as_slice())?);
                Ok(AST::Binary { left, op, right })
            }
        },
        ExpressionToken::String(s) => Ok(AST::Literal(Literal::Str(s.to_string()))),
        ExpressionToken::Boolean(b) => Ok(AST::Literal(Literal::Bool(b.clone()))),
        ExpressionToken::Integer(n) => Ok(AST::Literal(Literal::Int(n.clone()))),
        ExpressionToken::Ident(ident) => Ok(AST::Variable(ident.to_string())),
        ExpressionToken::XNode(n) => Ok(AST::Literal(Literal::XNode(n.clone()))),
        ExpressionToken::FuncCall(func) => Ok(AST::FuncCall {
            name: func.ident().to_string(),
            args: func
                .params()
                .iter()
                .map(|x| parse(std::slice::from_ref(x)))
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ExpressionToken::IfExpression {
            condition,
            then_branch,
            else_branch,
        } => Ok(AST::IfStatement {
            condition: token_to_ast(condition).map(|x| Box::new(x))?,
            then_branch: token_to_ast(then_branch).map(|x| Box::new(x))?,
            else_branch: match else_branch {
                Some(token) => Some(token_to_ast(token).map(|x| Box::new(x))?),
                None => None,
            },
        }),
        _ => Err(PySyntaxError::new_err(format!("Syntax error near {}", tok))),
    };
    ast
}

pub fn parse(tokens: &[ExpressionToken]) -> Result<AST, PyErr> {
    let mut iter = tokens.iter();
    let tok = iter
        .next()
        .ok_or(PySyntaxError::new_err("expected at least one token"))?;
    let mut left = token_to_ast(tok)?;

    while let Some(op_token) = iter.next() {
        let op = match op_token {
            ExpressionToken::Operator(op) => op.clone(),
            _ => {
                return Err(PySyntaxError::new_err(format!(
                    "Operator expected, got {}",
                    op_token,
                )))
            }
        };
        let right = token_to_ast(
            iter.next()
                .ok_or(PySyntaxError::new_err("token expected"))?,
        )?;

        left = AST::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn eval_add(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a + b)),
        (Literal::Int(a), Literal::Bool(b)) => Ok(Literal::Int(a + b as isize)),
        (Literal::Bool(a), Literal::Int(b)) => Ok(Literal::Int(a as isize + b)),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(Literal::Int(a as isize + b as isize)),
        (Literal::Str(a), Literal::Str(b)) => Ok(Literal::Str(a + &b)),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for addition",
        )),
    }
}

fn eval_sub(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a - b)),
        (Literal::Int(a), Literal::Bool(b)) => Ok(Literal::Int(a - b as isize)),
        (Literal::Bool(a), Literal::Int(b)) => Ok(Literal::Int(a as isize - b)),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(Literal::Int(a as isize - b as isize)),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for subtraction",
        )),
    }
}

fn eval_mul(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a * b)),
        (Literal::Int(a), Literal::Bool(b)) => Ok(Literal::Int(a * b as isize)),
        (Literal::Bool(a), Literal::Int(b)) => Ok(Literal::Int(a as isize * b)),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(Literal::Int(a as isize * b as isize)),
        (Literal::Str(a), Literal::Int(b)) => Ok(Literal::Str(if b > 0 {
            a.repeat(b as usize)
        } else {
            "".to_string()
        })),
        (Literal::Str(a), Literal::Bool(b)) => Ok(Literal::Str(a.repeat(b as usize))),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for multiplication",
        )),
    }
}

fn eval_div(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => {
            if b == 0 {
                Err(PyErr::new::<PyZeroDivisionError, _>("Division by zero"))
            } else {
                Ok(Literal::Int(a / b))
            }
        }
        (Literal::Int(a), Literal::Bool(b)) => {
            if b as isize == 0 {
                Err(PyErr::new::<PyZeroDivisionError, _>("Division by zero"))
            } else {
                Ok(Literal::Int(a / b as isize))
            }
        }
        (Literal::Bool(a), Literal::Int(b)) => {
            if b == 0 {
                Err(PyErr::new::<PyZeroDivisionError, _>("Division by zero"))
            } else {
                Ok(Literal::Int(a as isize / b))
            }
        }
        (Literal::Bool(a), Literal::Bool(b)) => {
            if b as isize == 0 {
                Err(PyErr::new::<PyZeroDivisionError, _>("Division by zero"))
            } else {
                Ok(Literal::Int(a as isize / b as isize))
            }
        }
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for division",
        )),
    }
}

fn eval_and(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l.is_truthy(), r.is_truthy()) {
        (true, false) => Ok(r),
        (false, false) => Ok(l),
        (false, true) => Ok(l),
        (true, true) => Ok(r),
    }
}

fn eval_or(l: Literal, r: Literal) -> PyResult<Literal> {
    match (l.is_truthy(), r.is_truthy()) {
        (true, false) => Ok(l),
        (false, false) => Ok(r),
        (false, true) => Ok(r),
        (true, true) => Ok(l),
    }
}

fn eval_raw_eq(l: Literal, r: Literal) -> PyResult<bool> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(a == b),
        (Literal::Int(a), Literal::Bool(b)) => Ok(a == b as isize),
        (Literal::Bool(a), Literal::Int(b)) => Ok(a as isize == b),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(a == b),
        (Literal::Str(a), Literal::Str(b)) => Ok(a == b),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for equality",
        )),
    }
}

fn eval_eq(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_eq(l, r).map(|b| Literal::Bool(b));
}

fn eval_neq(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_eq(l, r).map(|b| Literal::Bool(!b));
}

fn eval_raw_gt(l: Literal, r: Literal) -> PyResult<bool> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(a > b),
        (Literal::Int(a), Literal::Bool(b)) => Ok(a > b as isize),
        (Literal::Bool(a), Literal::Int(b)) => Ok(a as isize > b),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(a > b),
        (Literal::Str(a), Literal::Str(b)) => Ok(a > b),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for comparison",
        )),
    }
}

fn eval_gt(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_gt(l, r).map(|b| Literal::Bool(b));
}

pub fn eval_ast<'py>(
    py: Python<'py>,
    ast: &'py AST,
    catalog: &XCatalog,
    params: &HashMap<String, Literal>,
) -> Result<Literal, PyErr> {
    // error!(":::::::");
    // error!("{:?}", ast);
    match ast {
        AST::Literal(lit) => Ok(lit.clone()),

        AST::Binary { left, op, right } => {
            let l = eval_ast(py, left, catalog, params)?;
            let r = eval_ast(py, right, catalog, params)?;

            match op {
                Operator::Add => eval_add(l, r),
                Operator::Sub => eval_sub(l, r),
                Operator::Mul => eval_mul(l, r),
                Operator::Div => eval_div(l, r),
                Operator::And => eval_and(l, r),
                Operator::Or => eval_or(l, r),
                Operator::Eq => eval_eq(l, r),
                Operator::Neq => eval_neq(l, r),
                Operator::Gt => eval_gt(l, r),
            }
        }

        AST::Variable(name) => match params.get(name) {
            Some(Literal::Bool(v)) => Ok(Literal::Bool(v.clone())),
            Some(Literal::Int(v)) => Ok(Literal::Int(v.clone())),
            Some(Literal::Str(v)) => Ok(Literal::Str(v.clone())),
            Some(Literal::XNode(node)) => {
                let resp = catalog.render_node(py, node, PyDict::new(py));
                resp.map(|markup| Literal::Str(markup))
            }
            None => Err(PyErr::new::<pyo3::exceptions::PyUnboundLocalError, _>(
                format!("Undefined: {}", name),
            )),
        },

        AST::FuncCall { name, args } => {
            let lit_args = args
                .iter()
                .map(|arg| eval_ast(py, arg, catalog, params))
                .collect::<Result<Vec<_>, _>>()?;
            let py_args = PyTuple::new(py, lit_args)?;
            let res = catalog.call(py, name, &py_args)?;
            Literal::downcast(res)
        }

        AST::IfStatement {
            condition,
            then_branch,
            else_branch,
        } => {
            let is_then = eval_ast(py, condition, catalog, params)?;
            if is_then.is_truthy() {
                eval_ast(py, then_branch, catalog, params)
            } else {
                if let Some(else_) = else_branch {
                    eval_ast(py, else_, catalog, params)
                } else {
                    Ok(Literal::Str("".to_string()))
                }
            }
        }
    }
}

fn cast_params<'py>(params: Bound<'py, PyDict>) -> Result<HashMap<String, Literal>, PyErr> {
    let mut result = HashMap::new();

    for (key, value) in params.iter() {
        let key_str = key.downcast::<PyString>()?.to_string();
        let val = Literal::downcast(value)?;
        result.insert(key_str, val);
    }
    Ok(result)
}

pub fn eval_expression<'py>(
    py: Python<'py>,
    expression: &str,
    catalog: &XCatalog,
    params: Bound<'py, PyDict>,
) -> Result<Literal, PyErr> {
    info!(
        "Evaluating expression {}...",
        &expression[..min(expression.len(), 24)]
    );
    let params_ast = cast_params(params)?;
    let token = parse_expression(expression)?;
    let ast = parse(&[token])?;
    eval_ast(py, &ast, catalog, &params_ast)
}
