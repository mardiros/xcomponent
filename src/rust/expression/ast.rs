use std::cmp::min;
use std::collections::HashMap;
use std::fmt;

use pyo3::exceptions::{
    PyAttributeError, PyIndexError, PyKeyError, PySyntaxError, PyTypeError, PyZeroDivisionError,
};
use pyo3::types::{PyBool, PyDict, PyInt, PyList, PyString, PyTuple};
use pyo3::{prelude::*, BoundObject, IntoPyObjectExt};

use crate::catalog::XCatalog;
use crate::markup::tokens::{ToHtml, XNode};

use super::tokens::PostfixOp;
use super::{
    parser::tokenize,
    tokens::{ExpressionToken, Operator},
};

trait Truthy {
    fn is_truthy(&self) -> bool;
}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKey {
    Int(isize),
    Str(String),
    Uuid(String),
}

impl fmt::Display for LiteralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKey::Int(v) => write!(f, "{}", v),
            LiteralKey::Str(v) => write!(f, "\"{}\"", v),
            LiteralKey::Uuid(v) => write!(f, "\"{}\"", v),
        }
    }
}

impl LiteralKey {
    fn downcast<'py>(value: Bound<'py, PyAny>) -> Result<Self, PyErr> {
        if let Ok(v) = value.downcast::<PyInt>() {
            return Ok(LiteralKey::Int(v.extract::<isize>()?));
        } else if let Ok(v) = value.downcast::<PyString>() {
            return Ok(LiteralKey::Str(v.to_string()));
        } else if value.downcast::<PyAny>()?.get_type().name()? == "UUID" {
            let uuid_str = value.getattr("hex")?;
            Ok(LiteralKey::Uuid(uuid_str.to_string()))
        } else {
            let err: PyErr = PyTypeError::new_err(format!("Can't parse parameter {:?}", value));
            return Err(err);
        }
    }
    fn into_py<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, PyAny> {
        match self {
            LiteralKey::Int(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            LiteralKey::Uuid(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            LiteralKey::Str(v) => v.clone().into_pyobject(py).unwrap().into_any(),
        }
    }
}

impl TryFrom<Literal> for LiteralKey {
    type Error = PyErr;

    fn try_from(lit: Literal) -> Result<Self, Self::Error> {
        match lit {
            Literal::Int(i) => Ok(LiteralKey::Int(i)),
            Literal::Str(s) => Ok(LiteralKey::Str(s)),
            Literal::Uuid(u) => Ok(LiteralKey::Uuid(u)),
            _ => Err(PyTypeError::new_err(format!(
                "Unsupported literal type for key {:?}",
                lit
            ))),
        }
    }
}

// // equivalent to former `ToPyObject` implementations
// impl<'py> IntoPyObject<'py> for LiteralKey {
//     type Target = PyString;
//     type Output = Bound<'py, Self::Target>;
//     type Error = std::convert::Infallible;

//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         match self {
//             LiteralKey::Str(v) => v.clone().into_pyobject(py),
//             LiteralKey::Uuid(v) => v.clone().into_pyobject(py),
//         }
//     }
// }

#[derive(Debug, Clone, IntoPyObject)]
pub enum Literal {
    Bool(bool),
    Int(isize),
    Str(String),
    Uuid(String), // Uuid type does not support IntoPyObject
    XNode(XNode),
    List(Vec<Literal>),
    Dict(HashMap<LiteralKey, Literal>),
    Callable(String), // the name of the callable
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
        } else if let Ok(seq) = value.downcast::<PyList>() {
            let mut items = Vec::with_capacity(seq.len());
            for item in seq.iter() {
                items.push(Literal::downcast(item)?);
            }
            Ok(Literal::List(items))
        } else if let Ok(dict) = value.downcast::<PyDict>() {
            let mut map = HashMap::new();
            for (k, v) in dict {
                let key: LiteralKey = LiteralKey::downcast(k)?;
                let val: Literal = Literal::downcast(v)?;
                map.insert(key, val);
            }
            Ok(Literal::Dict(map))
        } else if value.downcast::<PyAny>()?.get_type().name()? == "UUID" {
            let uuid_str = value.getattr("hex")?;
            Ok(Literal::Uuid(uuid_str.to_string()))
        } else {
            let err: PyErr = PyTypeError::new_err(format!("Can't parse parameter {:?}", value));
            return Err(err);
        }
    }
    fn into_py<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, PyAny> {
        match self {
            Literal::Bool(v) => v // wtf
                .into_pyobject(py)
                .unwrap()
                .unbind()
                .into_bound_py_any(py)
                .unwrap(),
            Literal::Uuid(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            Literal::Int(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            Literal::Str(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            Literal::XNode(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            Literal::List(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            Literal::Callable(v) => v.clone().into_pyobject(py).unwrap().into_any(), // wrong!
            Literal::Dict(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(
                        k.clone().into_pyobject(py).unwrap().into_any(),
                        v.into_py(py),
                    )
                    .unwrap();
                }
                dict.into_any()
            }
        }
    }
}

impl Truthy for Literal {
    fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(bool) => bool.clone(),
            Literal::Int(i) => *i != 0,
            Literal::Str(s) => !s.is_empty(),
            Literal::Uuid(_) => true,
            Literal::XNode(_) => true,
            Literal::Callable(_) => true,
            Literal::List(items) => !items.is_empty(),
            Literal::Dict(d) => !d.is_empty(),
        }
    }
}

impl ToHtml for Literal {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        params: Bound<'py, PyDict>,
    ) -> PyResult<String> {
        match self {
            Literal::Bool(b) => Ok(format!("{}", b)),
            Literal::Int(i) => Ok(format!("{}", i)),
            Literal::Str(s) => Ok(format!("{}", s)),
            Literal::Callable(s) => Ok(format!("{}()", s)),
            Literal::Uuid(uuid) => Ok(format!("{}", uuid)),
            Literal::List(l) => {
                let mut out = String::new();
                for item in l {
                    out.push_str(item.to_html(py, catalog, params.clone())?.as_str());
                }
                Ok(out)
            }
            Literal::Dict(d) => {
                let mut out = String::new();
                out.push_str("<dl>");
                for (k, item) in d {
                    out.push_str("<dt>");
                    out.push_str(format!("{}", k).as_str());
                    out.push_str("</dt>");
                    out.push_str("<dt>");
                    out.push_str(item.to_html(py, catalog, params.clone())?.as_str());
                    out.push_str("</dt>");
                }
                out.push_str("</dl>");
                Ok(out)
            }
            Literal::XNode(n) => catalog.render_node(py, &n, params),
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
    FieldAccess(Box<AST>, String),
    IndexAccess(Box<AST>, Box<AST>),
    CallAccess {
        left: Box<AST>,
        args: Vec<AST>,
        kwargs: HashMap<String, AST>,
    },
    IfStatement {
        condition: Box<AST>,
        then_branch: Box<AST>,
        else_branch: Option<Box<AST>>,
    },
    ForStatement {
        ident: String,
        iterable: Box<AST>,
        body: Box<AST>,
    },
}

fn token_to_ast(tok: &ExpressionToken) -> Result<AST, PyErr> {
    let ast = match tok {
        ExpressionToken::BinaryExpression(ex) => Ok(parse(ex.as_slice())?),
        ExpressionToken::String(s) => Ok(AST::Literal(Literal::Str(s.to_string()))),
        // ExpressionToken::Uuid(s) => Ok(AST::Literal(Literal::Uuid(s.to_string()))),
        ExpressionToken::Boolean(b) => Ok(AST::Literal(Literal::Bool(b.clone()))),
        ExpressionToken::Integer(n) => Ok(AST::Literal(Literal::Int(n.clone()))),
        ExpressionToken::Ident(ident) => Ok(AST::Variable(ident.to_string())),
        ExpressionToken::XNode(n) => Ok(AST::Literal(Literal::XNode(n.clone()))),
        ExpressionToken::PostfixOp(op) => {
            // the ast is handled by the
            error!("Should never enter postfix op code : {:?}", op);
            Ok(AST::Literal(Literal::Str("".to_string())))
        }
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
        ExpressionToken::ForExpression {
            ident,
            iterable,
            body,
        } => Ok(AST::ForStatement {
            ident: ident.clone(),
            iterable: token_to_ast(iterable).map(|x| Box::new(x))?,
            body: token_to_ast(body).map(|x| Box::new(x))?,
        }),
        _ => Err(PySyntaxError::new_err(format!(
            "Syntax error, unexpected token {:?}",
            tok
        ))),
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
        match op_token {
            ExpressionToken::PostfixOp(op) => match op {
                PostfixOp::Field(f) => left = AST::FieldAccess(Box::new(left), f.clone()),
                PostfixOp::Index(i) => {
                    left = AST::IndexAccess(Box::new(left), Box::new(token_to_ast(i)?))
                }
                PostfixOp::Call { args, kwargs } => {
                    left = AST::CallAccess {
                        left: Box::new(left),
                        args: args
                            .into_iter()
                            .map(|arg| -> Result<_, _> { token_to_ast(arg) })
                            .collect::<Result<_, _>>()?,
                        kwargs: kwargs
                            .into_iter()
                            .map(|(k, v)| -> Result<(String, AST), PyErr> {
                                Ok((k.clone(), token_to_ast(v)?))
                            })
                            .collect::<Result<_, _>>()?,
                    };
                }
            },
            ExpressionToken::Operator(op) => {
                let right = token_to_ast(
                    iter.next()
                        .ok_or(PySyntaxError::new_err("token expected"))?,
                )?;

                left = AST::Binary {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                };
            }
            _ => {
                return Err(PySyntaxError::new_err(format!(
                    "Operator expected, got {:?}",
                    op_token,
                )))
            }
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

fn eval_raw_lt(l: Literal, r: Literal) -> PyResult<bool> {
    match (l, r) {
        (Literal::Int(a), Literal::Int(b)) => Ok(a < b),
        (Literal::Int(a), Literal::Bool(b)) => Ok(a < b as isize),
        (Literal::Bool(a), Literal::Int(b)) => Ok((a as isize) < b),
        (Literal::Bool(a), Literal::Bool(b)) => Ok(a < b),
        (Literal::Str(a), Literal::Str(b)) => Ok(a < b),
        _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid types for comparison",
        )),
    }
}

fn eval_gt(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_gt(l, r).map(|b| Literal::Bool(b));
}

fn eval_lt(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_lt(l, r).map(|b| Literal::Bool(b));
}

fn eval_gte(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_lt(l, r).map(|b| Literal::Bool(!b));
}

fn eval_lte(l: Literal, r: Literal) -> PyResult<Literal> {
    return eval_raw_gt(l, r).map(|b| Literal::Bool(!b));
}

pub fn eval_ast<'py>(
    py: Python<'py>,
    ast: &'py AST,
    catalog: &XCatalog,
    params: &HashMap<LiteralKey, Literal>,
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
                Operator::Lt => eval_lt(l, r),
                Operator::Gte => eval_gte(l, r),
                Operator::Lte => eval_lte(l, r),
            }
        }

        AST::Variable(name) => match params.get(&LiteralKey::Str(name.clone())) {
            Some(Literal::Bool(v)) => Ok(Literal::Bool(v.clone())),
            Some(Literal::Int(v)) => Ok(Literal::Int(v.clone())),
            Some(Literal::Str(v)) => Ok(Literal::Str(v.clone())),
            Some(Literal::Callable(v)) => Ok(Literal::Callable(v.clone())),
            Some(Literal::Uuid(v)) => Ok(Literal::Uuid(v.clone())),
            Some(Literal::List(v)) => Ok(Literal::List(v.clone())),
            Some(Literal::Dict(v)) => Ok(Literal::Dict(v.clone())),
            Some(Literal::XNode(node)) => {
                let resp = catalog.render_node(py, node, PyDict::new(py));
                resp.map(|markup| Literal::Str(markup))
            }
            None => {
                if let Some(_) = catalog.functions().get(name) {
                    Ok(Literal::Callable(name.clone()))
                } else {
                    Err(PyErr::new::<pyo3::exceptions::PyUnboundLocalError, _>(
                        format!("Undefined: {:?}", name),
                    ))
                }
            }
        },
        AST::FieldAccess(obj, field) => {
            let base = eval_ast(py, &obj, &catalog, &params)?;
            match base {
                Literal::Dict(map) => {
                    if let Some(val) = map.get(&LiteralKey::Str(field.clone())) {
                        return Ok(val.clone());
                    }
                    if let Some(val) = map.get(&LiteralKey::Uuid(field.clone())) {
                        return Ok(val.clone());
                    }
                    Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(format!(
                        "Field '{}' not found in {:?}",
                        field, map
                    )))
                }
                _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                    "Cannot access field '{}' on non-object",
                    field
                ))),
            }
        }

        AST::IndexAccess(obj, index) => {
            // obj[index]
            let base = eval_ast(py, obj, catalog, params)?;
            let key = eval_ast(py, index, catalog, params)?;
            match base {
                Literal::Dict(map) => {
                    let value = map
                        .get(&LiteralKey::try_from(key.clone())?)
                        .ok_or_else(|| PyKeyError::new_err(format!("{:?}", key)))?;
                    Ok(value.clone())
                }
                Literal::List(lst) => match key {
                    Literal::Int(idx) => {
                        let real_index = if idx > 0 {
                            idx as isize
                        } else {
                            (lst.len() as isize + idx) as isize
                        };
                        if real_index < 0 {
                            Err(PyIndexError::new_err(format!("Index out of range {}", idx)))
                        } else {
                            let value = lst.get(real_index as usize).ok_or_else(|| {
                                PyIndexError::new_err(format!("Index out of range {}", idx))
                            })?;
                            Ok(value.clone())
                        }
                    }
                    _ => Err(PyTypeError::new_err(format!("{:?}", key))),
                },
                _ => Err(PyErr::new::<PyTypeError, _>(format!(
                    "Cannot access index '{:?}' on non-object",
                    base
                ))),
            }
        }

        AST::CallAccess { left, args, kwargs } => {
            // left(*args, **kwargs)
            let base = eval_ast(py, left, catalog, params)?;

            let lit_args = args
                .iter()
                .map(|arg| eval_ast(py, arg, catalog, params))
                .collect::<Result<Vec<_>, _>>()?;

            let lit_kwargs = kwargs
                .iter()
                .map(|(name, arg)| Ok((name.clone(), eval_ast(py, arg, catalog, params)?)))
                .collect::<Result<HashMap<String, Literal>, PyErr>>()?;
            let py_args = PyTuple::new(py, lit_args.iter().map(|v| v.into_py(py)))?;
            let py_kwargs = PyDict::new(py);
            for (k, v) in lit_kwargs {
                py_kwargs.set_item(k, v.into_py(py))?;
            }
            match base {
                Literal::Callable(ident) => {
                    let res = catalog.call(py, ident.as_str(), &py_args, &py_kwargs)?;
                    Literal::downcast(res)
                }
                _ => Err(PyAttributeError::new_err(format!(
                    "{:?} is not callable",
                    base
                ))),
            }
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
        AST::ForStatement {
            ident,
            iterable,
            body,
        } => {
            let iter_lit = eval_ast(py, iterable, catalog, params)?;

            // let var = params.get(iterable).map(|x| Ok(x)).unwrap_or_else(|| {
            //     return Err(PyUnboundLocalError::new_err(format!(
            //         "{:?} is not defined in {:?}",
            //         ident, params
            //     )));
            // })?;
            match iter_lit {
                Literal::List(iter) => {
                    let mut res = String::new();
                    for v in iter {
                        let mut block_params = params.clone();
                        block_params.insert(LiteralKey::Str(ident.clone()), v);
                        let item = eval_ast(py, body, catalog, &block_params)?;
                        res.push_str(
                            item.to_html(py, catalog, wrap_params(py, &block_params)?)?
                                .as_str(),
                        )
                    }
                    Ok(Literal::Str(res))
                }
                _ => Err(PyTypeError::new_err(format!(
                    "{} {:?} is not iterable",
                    ident, iter_lit
                ))),
            }
        }
    }
}

fn cast_params<'py>(params: Bound<'py, PyDict>) -> Result<HashMap<LiteralKey, Literal>, PyErr> {
    let mut result = HashMap::new();
    for (key, value) in params.iter() {
        let key_str = LiteralKey::Str(key.to_string());
        let val = Literal::downcast(value)?;
        result.insert(key_str, val);
    }
    Ok(result)
}

fn wrap_params<'py>(
    py: Python<'py>,
    params: &HashMap<LiteralKey, Literal>,
) -> Result<Bound<'py, PyDict>, PyErr> {
    let result = PyDict::new(py);
    for (key, value) in params.iter() {
        result.set_item(key.into_py(py), value.into_py(py))?;
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
    let token = tokenize(expression)?;
    let ast = parse(&[token])?;
    eval_ast(py, &ast, catalog, &params_ast)
}
