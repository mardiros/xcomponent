use std::cmp::min;
use std::collections::HashMap;
use std::fmt;

use pyo3::exceptions::{PySyntaxError, PyTypeError, PyZeroDivisionError};
use pyo3::types::{PyBool, PyDict, PyInt, PyList, PyString, PyTuple};
use pyo3::{prelude::*, BoundObject, IntoPyObjectExt};

use crate::catalog::XCatalog;
use crate::markup::tokens::{ToHtml, XNode};

use super::{
    parser::parse_expression,
    tokens::{ExpressionToken, Operator},
};

trait Truthy {
    fn is_truthy(&self) -> bool;
}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKey {
    Str(String),
    Uuid(String),
}

impl fmt::Display for LiteralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKey::Str(v) => write!(f, "{}", v),
            LiteralKey::Uuid(v) => write!(f, "{}", v),
        }
    }
}

impl LiteralKey {
    fn downcast<'py>(value: Bound<'py, PyAny>) -> Result<Self, PyErr> {
        if let Ok(v) = value.downcast::<PyString>() {
            error!("<ssssssssssssss> {:?}", v.to_string());
            return Ok(LiteralKey::Str(v.to_string()));
        } else if value.downcast::<PyAny>()?.get_type().name()? == "UUID" {
            error!("<UUIDUUIDUUIDUUID> {:?}", value);
            let uuid_str = value.getattr("hex")?;
            Ok(LiteralKey::Uuid(uuid_str.to_string()))
        } else {
            error!("<><><><><><><><><><><><><><> {:?}", value);
            let err: PyErr = PyTypeError::new_err(format!("Can't parse parameter {:?}", value));
            return Err(err);
        }
    }
    fn into_py<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, PyAny> {
        match self {
            LiteralKey::Uuid(v) => v.clone().into_pyobject(py).unwrap().into_any(),
            LiteralKey::Str(v) => v.clone().into_pyobject(py).unwrap().into_any(),
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
    FuncCall {
        name: String,
        args: Vec<AST>,
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
        // ExpressionToken::Uuid(s) => Ok(AST::Literal(Literal::Uuid(s.to_string()))),
        ExpressionToken::Boolean(b) => Ok(AST::Literal(Literal::Bool(b.clone()))),
        ExpressionToken::Integer(n) => Ok(AST::Literal(Literal::Int(n.clone()))),
        ExpressionToken::Ident(ident) => Ok(AST::Variable(ident.to_string())),
        ExpressionToken::XNode(n) => Ok(AST::Literal(Literal::XNode(n.clone()))),
        ExpressionToken::FieldAccess(base, field) => {
            let base_ast = token_to_ast(base)?;
            Ok(AST::FieldAccess(Box::new(base_ast), field.clone()))
        }
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
        ExpressionToken::ForExpression {
            ident,
            iterable,
            body,
        } => Ok(AST::ForStatement {
            ident: ident.clone(),
            iterable: token_to_ast(iterable).map(|x| Box::new(x))?,
            body: token_to_ast(body).map(|x| Box::new(x))?,
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
            Some(Literal::Uuid(v)) => Ok(Literal::Uuid(v.clone())),
            Some(Literal::List(v)) => Ok(Literal::List(v.clone())),
            Some(Literal::Dict(v)) => Ok(Literal::Dict(v.clone())),
            Some(Literal::XNode(node)) => {
                let resp = catalog.render_node(py, node, PyDict::new(py));
                resp.map(|markup| Literal::Str(markup))
            }
            None => Err(PyErr::new::<pyo3::exceptions::PyUnboundLocalError, _>(
                format!("Undefined: {}", name),
            )),
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
                    Err(PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                        format!("Field '{}' not found", field),
                    ))
                }
                _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                    "Cannot access field '{}' on non-object",
                    field
                ))),
            }
        }

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
    let token = parse_expression(expression)?;
    let ast = parse(&[token])?;
    eval_ast(py, &ast, catalog, &params_ast)
}
