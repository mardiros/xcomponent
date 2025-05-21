use std::cmp::min;
use std::collections::HashMap;

use pyo3::exceptions::{
    PyAttributeError, PyIndexError, PyKeyError, PyTypeError, PyZeroDivisionError,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::catalog::XCatalog;
use crate::expression::ast::parse::parse;
use crate::expression::{parser::tokenize, tokens::Operator};
use crate::markup::tokens::ToHtml;

use super::model::{Literal, LiteralKey, Truthy, AST};

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
    globals: &HashMap<LiteralKey, Literal>,
) -> Result<Literal, PyErr> {
    // error!(":::::::");
    // error!("{:?}", ast);
    match ast {
        AST::Literal(lit) => Ok(lit.clone()),

        AST::Binary { left, op, right } => {
            let l = eval_ast(py, left, catalog, params, &globals)?;
            let r = eval_ast(py, right, catalog, params, &globals)?;

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
            Some(Literal::Object(v)) => Ok(Literal::Object(v.clone())),
            Some(Literal::XNode(node)) => {
                let resp =
                    catalog.render_node(py, node, PyDict::new(py), wrap_params(py, globals)?);
                resp.map(|markup| Literal::Str(markup))
            }
            None => {
                let k = LiteralKey::Str(name.clone());
                if let Some(lit) = globals.get(&k) {
                    Ok(lit.clone())
                } else if let Some(_) = catalog.functions().get(name) {
                    Ok(Literal::Callable(name.clone()))
                } else {
                    Err(PyErr::new::<pyo3::exceptions::PyUnboundLocalError, _>(
                        format!("{:?} is undefined", name),
                    ))
                }
            }
        },
        AST::FieldAccess(obj, field) => {
            let base = eval_ast(py, &obj, &catalog, &params, &globals)?;
            match base {
                Literal::Dict(map) => {
                    // no integer cannot be a field name here
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
                Literal::Object(o) => Python::with_gil(|py| {
                    // only string here. maybe callable
                    let item = o.obj().getattr(py, field)?.into_bound(py);
                    Literal::downcast(item)
                }),
                _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                    "Cannot access field '{}' on non-object",
                    field
                ))),
            }
        }

        AST::IndexAccess(obj, index) => {
            // obj[index]
            let base = eval_ast(py, obj, catalog, params, &globals)?;
            let key = eval_ast(py, index, catalog, params, &globals)?;
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
                Literal::Object(o) => Python::with_gil(|py| {
                    let item = match key {
                        Literal::Int(idx) => {
                            // FIXME, add len call here for negatif index
                            o.obj().into_pyobject(py).unwrap().call_method(
                                "__getitem__",
                                (idx,),
                                None,
                            )
                        }
                        _ => Err(PyTypeError::new_err(format!("Index access{:?}", key))),
                    }?;
                    Literal::downcast(item)
                }),
                _ => Err(PyErr::new::<PyTypeError, _>(format!(
                    "Cannot access index '{:?}' on non-object",
                    base
                ))),
            }
        }

        AST::CallAccess { left, args, kwargs } => {
            // left(*args, **kwargs)
            let base = eval_ast(py, left, catalog, params, &globals)?;

            let lit_args = args
                .iter()
                .map(|arg| eval_ast(py, arg, catalog, params, &globals))
                .collect::<Result<Vec<_>, _>>()?;

            let lit_kwargs = kwargs
                .iter()
                .map(|(name, arg)| {
                    Ok((name.clone(), eval_ast(py, arg, catalog, params, &globals)?))
                })
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
                Literal::Object(o) => Python::with_gil(|py| {
                    let res = o.obj().call(py, py_args, Some(&py_kwargs))?;
                    Literal::downcast(res.into_bound(py))
                }),
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
            let is_then = eval_ast(py, condition, catalog, params, &globals)?;
            if is_then.is_truthy() {
                eval_ast(py, then_branch, catalog, params, &globals)
            } else {
                if let Some(else_) = else_branch {
                    eval_ast(py, else_, catalog, params, &globals)
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
            let iter_lit = eval_ast(py, iterable, catalog, params, &globals)?;

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
                        let item = eval_ast(py, body, catalog, &block_params, &globals)?;
                        res.push_str(
                            item.to_html(
                                py,
                                catalog,
                                wrap_params(py, &block_params)?,
                                wrap_params(py, globals)?,
                            )?
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

pub(crate) fn cast_params<'py>(
    params: Bound<'py, PyDict>,
) -> Result<HashMap<LiteralKey, Literal>, PyErr> {
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
    globals: Bound<'py, PyDict>,
) -> Result<Literal, PyErr> {
    info!(
        "Evaluating expression {}...",
        &expression[..min(expression.len(), 24)]
    );
    let params_ast = cast_params(params)?;
    let token = tokenize(expression)?;
    let ast = parse(&[token])?;
    let global_params = cast_params(globals)?;
    eval_ast(py, &ast, catalog, &params_ast, &global_params)
}
