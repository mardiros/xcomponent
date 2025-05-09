use std::collections::HashMap;

use pyo3::{prelude::*, types::PyDict};

use crate::markup::{
    parser::parse_markup,
    tokens::{ToHtml, XNode},
};

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
    catalog: HashMap<String, Py<XTemplate>>,
}

#[pymethods]
impl XCatalog {
    #[new]
    pub fn new() -> Self {
        XCatalog {
            catalog: HashMap::new(),
        }
    }

    pub fn register<'py>(
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
        self.catalog.insert(name.to_owned(), py_template);
        Ok(())
    }

    pub fn get<'py>(&'py self, py: Python<'py>, name: &'py str) -> Option<&Bound<'py, XTemplate>> {
        self.catalog.get(name).map(|node| node.bind(py))
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
