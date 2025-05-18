use std::collections::HashMap;

use pyo3::{
    prelude::*,
    types::{PyAny, PyDict, PyTuple},
};

use crate::markup::{
    parser::parse_markup,
    tokens::{ToHtml, XNode},
};

#[pyclass]
pub struct PyCallable {
    callable: Py<PyAny>,
}

#[pymethods]
impl PyCallable {
    #[new]
    fn new(callable: Py<PyAny>) -> Self {
        PyCallable { callable }
    }

    fn call<'py>(
        &self,
        py: Python<'py>,
        args: Py<PyTuple>,
        kwargs: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.callable.bind(py).call(args, Some(kwargs))
    }
}

#[pyclass]
#[derive(Debug)]
pub struct XTemplate {
    node: Py<XNode>,
    params: Py<PyDict>,
}

#[pymethods]
impl XTemplate {
    #[new]
    pub fn new(node: Py<XNode>, params: Py<PyDict>) -> Self {
        XTemplate { node, params }
    }

    #[getter]
    pub fn node<'py>(&self, py: Python<'py>) -> &Bound<'py, XNode> {
        self.node.bind(py)
    }

    #[getter]
    pub fn params<'py>(&self, py: Python<'py>) -> &Bound<'py, PyAny> {
        self.params.bind(py)
    }
}

#[pyclass]
pub struct XCatalog {
    components: HashMap<String, Py<XTemplate>>,
    functions: HashMap<String, Py<PyCallable>>,
}

#[pymethods]
impl XCatalog {
    #[new]
    pub fn new() -> Self {
        XCatalog {
            components: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_component<'py>(
        &mut self,
        py: Python<'py>,
        name: &str,
        template: &str,
        params: Py<PyDict>,
    ) -> PyResult<()> {
        let node = parse_markup(template)?;
        let py_node = Py::new(py, node)?;
        let template = XTemplate::new(py_node, params);
        info!("Registering node {}", name);
        debug!("{:?}", template);
        let py_template = Py::new(py, template)?;
        self.components.insert(name.to_owned(), py_template);
        Ok(())
    }

    fn add_function<'py>(
        &mut self,
        py: Python<'py>,
        name: String,
        function: Py<PyAny>,
    ) -> PyResult<()> {
        info!("Registering function {}", name);
        debug!("{:?}", function);
        let func = PyCallable::new(function);
        let py_func = Py::new(py, func)?;
        self.functions.insert(name, py_func);
        Ok(())
    }

    pub fn get<'py>(&'py self, py: Python<'py>, name: &'py str) -> Option<&Bound<'py, XTemplate>> {
        self.components.get(name).map(|node| node.bind(py))
    }

    pub fn functions(&self) -> &HashMap<String, Py<PyCallable>> {
        &self.functions
    }
    pub fn call<'py>(
        &self,
        py: Python<'py>,
        name: &str,
        args: &Bound<'py, PyTuple>,
        kwargs: &Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let func = self
            .functions
            .get(name)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Function not found"))?;
        let res = func.bind(py).call_method("call", (args, kwargs), None);
        res
    }

    pub fn render_node<'py>(
        &self,
        py: Python<'py>,
        node: &XNode,
        params: Bound<'py, PyDict>,
    ) -> PyResult<String> {
        node.to_html(py, &self, params)
    }

    pub fn render<'py>(
        &self,
        py: Python<'py>,
        template: &str,
        params: Bound<'py, PyDict>,
    ) -> PyResult<String> {
        let node = parse_markup(template)?;
        self.render_node(py, &node, params)
    }
}
