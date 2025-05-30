use std::collections::HashMap;
use std::fmt;

use pyo3::exceptions::PyTypeError;
use pyo3::marker::Python;
use pyo3::types::{PyBool, PyDict, PyInt, PyList, PyNone, PyString};
use pyo3::{prelude::*, BoundObject, IntoPyObjectExt};

use crate::catalog::XCatalog;
use crate::markup::tokens::{ToHtml, XNode};

pub trait Truthy {
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
    pub fn downcast<'py>(py: Python<'py>, value: Bound<'py, PyAny>) -> Result<Self, PyErr> {
        if let Ok(v) = value.downcast::<LiteralKey>() {
            return Ok(v.as_unbound().extract(py).unwrap());
        } else if let Ok(v) = value.downcast::<PyInt>() {
            return Ok(LiteralKey::Int(v.extract::<isize>()?));
        } else if let Ok(v) = value.downcast::<PyString>() {
            return Ok(LiteralKey::Str(v.to_string()));
        } else if value.downcast::<PyAny>()?.get_type().name()? == "UUID" {
            let uuid_str = value.getattr("hex")?;
            Ok(LiteralKey::Uuid(uuid_str.to_string()))
        } else {
            let err: PyErr = PyTypeError::new_err(format!("Can't use {:?} as LiteralKey", value));
            return Err(err);
        }
    }
    pub fn into_py<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, PyAny> {
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

#[derive(Debug, IntoPyObject)]
pub struct PyObj {
    obj: Py<PyAny>,
}

impl PyObj {
    pub fn new(obj: Py<PyAny>) -> Self {
        PyObj { obj }
    }
    pub fn obj(&self) -> &Py<PyAny> {
        &self.obj
    }
}

impl Clone for PyObj {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyObj {
            obj: self.obj.clone_ref(py),
        })
    }
}

#[derive(Debug, Clone, IntoPyObject)]
pub enum Literal {
    None(()),
    Bool(bool),
    Int(isize),
    Str(String),
    Uuid(String), // Uuid type does not support IntoPyObject
    XNode(XNode),
    List(Vec<Literal>),
    Dict(HashMap<LiteralKey, Literal>),
    Callable(String), // the name of the callable
    Object(PyObj),
}

impl Literal {
    pub fn downcast<'py>(py: Python<'py>, value: Bound<'py, PyAny>) -> Result<Self, PyErr> {
        if let Ok(v) = value.downcast::<PyString>() {
            return Ok(Literal::Str(v.to_string()));
        } else if let Ok(v) = value.downcast::<PyBool>() {
            return Ok(Literal::Bool(v.extract::<bool>()?));
        } else if let Ok(v) = value.downcast::<PyInt>() {
            return Ok(Literal::Int(v.extract::<isize>()?));
        } else if let Ok(_) = value.downcast::<PyNone>() {
            return Ok(Literal::None(()));
        } else if let Ok(v) = value.extract::<XNode>() {
            return Ok(Literal::XNode(v));
        } else if let Ok(seq) = value.downcast::<PyList>() {
            let mut items = Vec::with_capacity(seq.len());
            for item in seq.iter() {
                items.push(Literal::downcast(py, item)?);
            }
            Ok(Literal::List(items))
        } else if let Ok(dict) = value.downcast::<PyDict>() {
            let mut map = HashMap::new();
            for (k, v) in dict {
                let key: LiteralKey = LiteralKey::downcast(py, k)?;
                let val: Literal = Literal::downcast(py, v)?;
                map.insert(key, val);
            }
            Ok(Literal::Dict(map))
        } else if value.downcast::<PyAny>()?.get_type().name()? == "UUID" {
            let uuid_str = value.getattr("hex")?;
            Ok(Literal::Uuid(uuid_str.to_string()))
        } else {
            let o: Py<PyAny> = value.extract()?;
            Ok(Literal::Object(PyObj::new(o)))
        }
    }
    pub fn into_py<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, PyAny> {
        let ret = match self {
            Literal::None(_) => Python::None(py).into_pyobject(py).unwrap().into_any(),
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
            Literal::List(v) => {
                let vals = v
                    .iter()
                    .map(|o| o.into_py(py))
                    .collect::<Vec<pyo3::Bound<'py, pyo3::PyAny>>>();
                let lst = PyList::new(py, vals).unwrap();
                lst.into_any()

                // v.clone().into_pyobject(py).unwrap().into_any()
            }
            Literal::Callable(v) => v.clone().into_pyobject(py).unwrap().into_any(), // wrong!
            Literal::Object(v) => v.clone().obj.into_pyobject(py).unwrap().into_any(),
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
        };
        ret
    }
}

impl Truthy for Literal {
    fn is_truthy(&self) -> bool {
        match self {
            Literal::None(_s) => false,
            Literal::Bool(bool) => bool.clone(),
            Literal::Int(i) => *i != 0,
            Literal::Str(s) => !s.is_empty(),
            Literal::Uuid(_) => true,
            Literal::XNode(_) => true,
            Literal::Callable(_) => true,
            Literal::Object(o) => {
                Python::with_gil(|py| {
                    match o
                        .obj()
                        .into_pyobject(py)
                        .unwrap()
                        .call_method("__bool__", (), None)
                    {
                        Ok(b) => b.extract::<bool>().unwrap(),
                        Err(_) => false, // or panic/log
                    }
                })
            }
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
        context: &mut RenderContext,
    ) -> PyResult<String> {
        debug!("Rendering {:?}", self);
        match self {
            Literal::None(_) => Ok("".to_string()),
            Literal::Bool(b) => Ok(format!("{}", b)),
            Literal::Int(i) => Ok(format!("{}", i)),
            Literal::Str(s) => Ok(format!("{}", s)),
            Literal::Callable(s) => Ok(format!("{}()", s)),
            Literal::Uuid(uuid) => Ok(format!("{}", uuid)),
            Literal::List(l) => {
                let mut out = String::new();
                for item in l {
                    out.push_str(item.to_html(py, catalog, context)?.as_str());
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
                    out.push_str(item.to_html(py, catalog, context)?.as_str());
                    out.push_str("</dt>");
                }
                out.push_str("</dl>");
                Ok(out)
            }
            Literal::Object(o) => Ok(format!(
                "{}",
                Python::with_gil(|py| {
                    match o
                        .obj()
                        .into_pyobject(py)
                        .unwrap()
                        .call_method("__repr__", (), None)
                    {
                        Ok(b) => b.extract::<String>().unwrap(),
                        Err(_) => "<PyObject>".to_string(),
                    }
                })
            )),
            Literal::XNode(n) => catalog.render_node(py, &n, context),
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct RenderContext {
    stack: Vec<HashMap<LiteralKey, Literal>>,
}

#[pymethods]
impl RenderContext {
    #[new]
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push<'py>(&mut self, py: Python<'py>, params: Bound<'py, PyDict>) -> PyResult<()> {
        let anyparams: Bound<'py, PyAny> = params.extract()?;
        if let Literal::Dict(d) = Literal::downcast(py, anyparams)? {
            self.stack.push(d);
            Ok(())
        } else {
            // we comme from a Pydict, so this is dead code, right?
            Err(PyTypeError::new_err(format!("Invalid rendering type")))
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

impl RenderContext {
    pub fn insert(&mut self, key: LiteralKey, value: Literal) {
        let mut d = HashMap::new();
        d.insert(key, value);
        self.stack.push(d);
    }
    pub fn get(&self, key: &LiteralKey) -> Option<&Literal> {
        self.stack.iter().rev().find_map(|scope| scope.get(key))
    }
}
