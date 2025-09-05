use std::collections::HashMap;

use pyo3::{prelude::*, types::PyDict};

use crate::{
    catalog::XCatalog,
    context::{Literal, LiteralKey, RenderContext},
    expression::ast::eval::eval_expression,
};

pub trait ToHtml {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<String>;
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    DocType,
    Fragment,
    ScriptElement,
    Element,
    Expression,
    Text,
    Comment,
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XFragment {
    children: Vec<XNode>,
}

#[pymethods]
impl XFragment {
    #[new]
    pub fn new(children: Vec<XNode>) -> Self {
        XFragment { children }
    }

    #[getter]
    fn children(&self) -> Vec<XNode> {
        self.children.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("children",)
    }
}

impl ToHtml for XFragment {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<String> {
        let mut result = String::new();
        for child in self.children() {
            result.push_str(child.to_html(py, catalog, context)?.as_str())
        }
        Ok(result)
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XScriptElement {
    name: String,
    attrs: HashMap<String, XNode>,
    body: String,
}
#[pymethods]
impl XScriptElement {
    #[new]
    pub fn new(name: String, attrs: HashMap<String, XNode>, body: String) -> Self {
        XScriptElement { name, attrs, body }
    }

    #[getter]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    #[getter]
    fn attrs(&self) -> HashMap<String, XNode> {
        self.attrs.clone()
    }

    #[getter]
    fn body(&self) -> &str {
        self.body.as_str()
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("name", "attrs", "body")
    }
}

impl ToHtml for XScriptElement {
    fn to_html<'py>(
        &self,
        _py: Python<'py>,
        _catalog: &XCatalog,
        _context: &mut RenderContext,
    ) -> PyResult<String> {
        let mut result = String::new();

        let joined_attrs = self
            .attrs()
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, v.__repr__()))
            .collect::<String>();

        result.push_str(format!("<{}{}>", self.name(), joined_attrs).as_str());
        result.push_str(format!("{}", self.body()).as_str());
        // The body ends wigh the closing tag, we don't need to add it.
        // result.push_str(format!("</{}>", self.name()).as_str());

        Ok(result)
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XElement {
    name: String,
    attrs: HashMap<String, XNode>,
    children: Vec<XNode>,
}

#[pymethods]
impl XElement {
    #[new]
    pub fn new(name: String, attrs: HashMap<String, XNode>, children: Vec<XNode>) -> Self {
        XElement {
            name,
            attrs,
            children,
        }
    }

    #[getter]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    #[getter]
    fn attrs(&self) -> HashMap<String, XNode> {
        self.attrs.clone()
    }

    #[getter]
    fn children(&self) -> Vec<XNode> {
        self.children.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str, &'static str) {
        ("name", "attrs", "children")
    }
}

#[inline]
fn render_attr<'py>(
    py: Python<'py>,
    catalog: &XCatalog,
    node: &XNode,
    name: &str,
    context: &mut RenderContext,
) -> PyResult<String> {
    let value = catalog.render_node(py, &node, context)?;
    let attr = if value.contains('"') {
        format!(" {}='{}'", name, value.replace('\'', "\\'"))
    } else {
        format!(" {}=\"{}\"", name, value)
    };
    Ok(attr)
}

impl ToHtml for XElement {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<String> {
        let mut result = String::new();

        match catalog.get(py, self.name()) {
            Some(py_template) => {
                debug!("Rendering template {}", py_template);

                let node = py_template.getattr("node")?.extract::<XNode>()?;
                let node_attrs = py_template
                    .getattr("defaults")?
                    .downcast::<PyDict>()?
                    .copy()?;

                for (name, attrnode) in self.attrs() {
                    let name = match name.as_str() {
                        "class" => "class_".to_string(),
                        "for" => "for_".to_string(),
                        _ => name.replace('-', "_"),
                    };
                    if let XNode::Expression(ref expression) = attrnode {
                        let node_attr_v =
                            eval_expression(py, expression.expression(), &catalog, context)?;
                        node_attrs.set_item(name, node_attr_v.into_py(py))?;
                    } else {
                        node_attrs.set_item(
                            name,
                            Literal::Str(catalog.render_node(py, &attrnode, context)?),
                        )?;
                    }
                }
                debug!("Rendered node_attrs {:?}", node_attrs);
                if self.children().len() > 0 {
                    let mut childchildren = String::new();
                    for child in self.children() {
                        childchildren.push_str(child.to_html(py, catalog, context)?.as_str())
                    }
                    node_attrs.set_item("children", childchildren)?;
                }

                let gblk = LiteralKey::Str("globals".to_string());
                let mut shadow_context = RenderContext::new();
                if let Some(glb) = context.get(&gblk) {
                    shadow_context.insert(gblk, glb.clone());
                }
                shadow_context.push(py, node_attrs)?;
                result.push_str(
                    catalog
                        .render_node(py, &node, &mut shadow_context)?
                        .as_str(),
                );
            }
            None => {
                debug!("Rendering final element <{}/>", self.name);
                result.push_str(format!("<{}", self.name).as_str());
                for (name, node) in self.attrs() {
                    let attr = match node {
                        XNode::Expression(ref expr) => {
                            let v = eval_expression(py, expr.expression(), &catalog, context)?;
                            match v {
                                Literal::None(()) => "".to_string(),
                                Literal::Bool(false) => "".to_string(),
                                Literal::Bool(true) => format!(" {}", name),
                                _ => render_attr(py, catalog, &node, name.as_str(), context)?,
                            }
                        }
                        _ => render_attr(py, catalog, &node, name.as_str(), context)?,
                    };
                    result.push_str(format!("{}", attr).as_str());
                }
                if self.children().len() > 0 {
                    result.push_str(">");
                    for child in self.children() {
                        result.push_str(child.to_html(py, catalog, context)?.as_str())
                    }
                    result.push_str(format!("</{}>", self.name).as_str());
                } else {
                    result.push_str("/>");
                }
            }
        }
        Ok(result)
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XDocType {
    doctype: String,
}

#[pymethods]
impl XDocType {
    #[new]
    pub fn new(doctype: String) -> Self {
        XDocType { doctype }
    }

    #[getter]
    fn doctype(&self) -> &str {
        self.doctype.as_str()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("doctype",)
    }
}

impl ToHtml for XDocType {
    fn to_html<'py>(
        &self,
        _: Python<'py>,
        _: &XCatalog,
        _: &mut RenderContext,
    ) -> PyResult<String> {
        Ok(format!("{}", self.doctype()))
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XComment {
    comment: String,
}

#[pymethods]
impl XComment {
    #[new]
    pub fn new(comment: String) -> Self {
        XComment { comment }
    }

    #[getter]
    fn comment(&self) -> &str {
        self.comment.as_str()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("comment",)
    }
}

impl ToHtml for XComment {
    fn to_html<'py>(
        &self,
        _: Python<'py>,
        _: &XCatalog,
        _: &mut RenderContext,
    ) -> PyResult<String> {
        Ok(format!(
            "<!--{}-->",
            html_escape::encode_text(self.comment())
        ))
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XText {
    text: String,
}

#[pymethods]
impl XText {
    #[new]
    pub fn new(text: String) -> Self {
        XText { text }
    }

    #[getter]
    fn text(&self) -> &str {
        self.text.as_str()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("text",)
    }
}

impl ToHtml for XText {
    fn to_html<'py>(
        &self,
        _: Python<'py>,
        _: &XCatalog,
        _: &mut RenderContext,
    ) -> PyResult<String> {
        return Ok(html_escape::encode_text(self.text()).to_string());
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq)]
pub struct XExpression {
    expression: String,
}

#[pymethods]
impl XExpression {
    #[new]
    pub fn new(expression: String) -> Self {
        XExpression { expression }
    }

    #[getter]
    fn expression(&self) -> &str {
        self.expression.as_str()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("expression",)
    }

    fn to_literal<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<Literal> {
        eval_expression(py, self.expression(), catalog, context)
    }
}

impl ToHtml for XExpression {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<String> {
        info!("Evaluating expression {}", self.expression());
        debug!("{:?}", context);
        let res = self.to_literal(py, catalog, context)?;
        res.to_html(py, catalog, context)
    }
}

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum XNode {
    Fragment(XFragment),
    ScriptElement(XScriptElement),
    Element(XElement),
    DocType(XDocType),
    Text(XText),
    Comment(XComment),
    Expression(XExpression),
}

impl std::fmt::Display for XNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XNode::Element(XElement {
                name,
                attrs,
                children,
            }) => {
                let joined_attrs = attrs
                    .iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, v.__repr__()))
                    .collect::<String>();

                if children.is_empty() {
                    write!(f, "<{}{}/>", name, joined_attrs)
                } else {
                    write!(f, "<{}{}>", name, joined_attrs)?;
                    for child in children {
                        write!(f, "{}", child)?;
                    }
                    write!(f, "</{}>", name)
                }
            }
            XNode::ScriptElement(XScriptElement { name, attrs, body }) => {
                let joined_attrs = attrs
                    .iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, v.__repr__()))
                    .collect::<String>();

                write!(f, "<{}{}>", name, joined_attrs)?;
                write!(f, "{}", body)?;
                write!(f, "</{}>", name)
            }

            XNode::Fragment(XFragment { children }) => {
                write!(
                    f,
                    "<>{}</>",
                    children.iter().map(|v| v.__repr__()).collect::<String>()
                )
            }
            XNode::DocType(XDocType { doctype }) => write!(f, "{}", doctype),
            XNode::Text(XText { text }) => write!(f, "{}", text),
            XNode::Comment(XComment { comment }) => write!(f, "<!--{}-->", comment),
            XNode::Expression(XExpression { expression }) => write!(f, "{{{}}}", expression),
        }
    }
}

#[pymethods]
impl XNode {
    #[getter]
    fn kind(&self) -> NodeType {
        match self {
            XNode::Fragment(_) => NodeType::Fragment,
            XNode::ScriptElement(_) => NodeType::ScriptElement,
            XNode::Element(_) => NodeType::Element,
            XNode::DocType(_) => NodeType::DocType,
            XNode::Text(_) => NodeType::Text,
            XNode::Comment(_) => NodeType::Comment,
            XNode::Expression(_) => NodeType::Expression,
        }
    }

    fn __repr__(&self) -> String {
        format!("{}", self)
    }
    fn __eq__(&self, other: &XNode) -> bool {
        self == other
    }

    #[pyo3(signature = ())]
    pub fn unwrap(&self, py: Python<'_>) -> PyObject {
        match self {
            XNode::Fragment(children) => children
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
            XNode::ScriptElement(element) => element
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
            XNode::Element(element) => element
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
            XNode::DocType(doctype) => doctype
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
            XNode::Comment(comment) => comment
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
            XNode::Text(text) => text.clone().into_pyobject(py).unwrap().into_any().unbind(),
            XNode::Expression(expression) => expression
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
        }
    }
}

impl<'py> IntoPyObject<'py> for &'py XNode {
    type Target = XNode;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl ToHtml for XNode {
    fn to_html<'py>(
        &self,
        py: Python<'py>,
        catalog: &XCatalog,
        context: &mut RenderContext,
    ) -> PyResult<String> {
        match self {
            XNode::Fragment(f) => f.to_html(py, catalog, context),
            XNode::Element(e) => e.to_html(py, catalog, context),
            XNode::ScriptElement(e) => e.to_html(py, catalog, context),
            XNode::DocType(d) => d.to_html(py, catalog, context),
            XNode::Text(t) => t.to_html(py, catalog, context),
            XNode::Comment(c) => c.to_html(py, catalog, context),
            XNode::Expression(e) => e.to_html(py, catalog, context),
        }
    }
}
