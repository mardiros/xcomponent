use std::collections::HashMap;

use pyo3::prelude::*;

use proc_macro2::TokenStream;
use quote::ToTokens;
use rstml::node::{Node, NodeAttribute};
use rstml::{parse2, Parser, ParserConfig};

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element = 1,
    Text = 3,
}

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
struct XNode {
    #[pyo3(get)]
    node_type: NodeType,
    #[pyo3(get)]
    tag: Option<String>,

    #[pyo3(get)]
    text: Option<String>,

    #[pyo3(get)]
    attrs: Option<HashMap<String, XNode>>,
    #[pyo3(get)]
    children: Option<Vec<XNode>>,
}

#[pymethods]
impl XNode {
    #[new]
    #[pyo3(signature = (node_type = NodeType::Element, tag = None, text = None, attrs = None, children = None))]
    fn new(
        node_type: NodeType,
        tag: Option<String>,
        text: Option<String>,
        attrs: Option<HashMap<String, XNode>>,
        children: Option<Vec<XNode>>,
    ) -> Self {
        XNode {
            node_type,
            tag,
            text,
            attrs,
            children,
        }
    }

    fn __repr__(&self) -> String {
        match (
            &self.node_type,
            &self.tag,
            &self.text,
            &self.attrs,
            &self.children,
        ) {
            (NodeType::Element, Some(tag), None, None, None) => format!("<{} />", tag),
            (NodeType::Element, Some(tag), None, Some(attrs), None) => {
                let joined_attrs = attrs
                    .iter()
                    .map(|(k, v)| match v.node_type {
                        NodeType::Text => format!(" {}=\"{}\"", k, v.__repr__()),
                        _ => format!(" {}={{{}}}", k, v.__repr__()),
                    })
                    .collect::<String>();
                format!("<{}{}/>", tag, joined_attrs)
            }
            (NodeType::Element, Some(tag), None, None, Some(children)) => {
                let joined_children: Vec<String> =
                    children.iter().map(|child| child.__repr__()).collect();
                format!("<{}>{}</{}>", tag, joined_children.join(""), tag)
            }
            (NodeType::Element, Some(tag), None, Some(attrs), Some(children)) => {
                let joined_attrs = attrs
                    .iter()
                    .map(|(k, v)| match v.node_type {
                        NodeType::Text => format!(" {}=\"{}\"", k, v.__repr__()),
                        _ => format!(" {}={{{}}}", k, v.__repr__()),
                    })
                    .collect::<String>();
                let joined_children: Vec<String> =
                    children.iter().map(|child| child.__repr__()).collect();
                format!(
                    "<{}{}>{}</{}>",
                    tag,
                    joined_attrs,
                    joined_children.join(""),
                    tag
                )
            }
            (NodeType::Text, None, Some(text), None, _) => text.to_owned(),
            _ => format!("XXX{:?}XXX", self), // we should never have something else here
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.node_type == other.node_type
            && self.tag == other.tag
            && self.text == other.text
            && self.attrs == other.attrs
            && self.children == other.children
    }
}

#[pyfunction]
fn parse_str(raw: &str) -> PyResult<XNode> {
    let tokens: TokenStream = raw.parse().map_err(|e: proc_macro2::LexError| {
        pyo3::exceptions::PyValueError::new_err(e.to_string())
    })?;

    let cfg = ParserConfig::new().number_of_top_level_nodes(1);
    let parser = Parser::new(cfg);

    let nodes = parser
        .parse_simple(tokens)
        .map_err(|e: syn::Error| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let out: Vec<XNode> = nodes.iter().filter_map(ast_to_xnode).collect();
    let value: XNode = out.first().unwrap().to_owned();
    Ok(value)
}

fn ast_to_xnode(node: &Node) -> Option<XNode> {
    match node {
        Node::Element(el) => {
            let tag = el.name().to_string();

            let mut attributes: HashMap<String, XNode> = HashMap::new();
            for attr in el.attributes() {
                match attr {
                    NodeAttribute::Block(_) => (),
                    NodeAttribute::Attribute(k) => {
                        let tokens: TokenStream = k.value().unwrap().to_token_stream();
                        let nodes = parse2(tokens)
                            .map_err(|e: syn::Error| {
                                pyo3::exceptions::PyValueError::new_err(e.to_string())
                            })
                            .unwrap(); // fixme
                        let out: Vec<XNode> = nodes.iter().filter_map(ast_to_xnode).collect();
                        let key = k.key.to_string();
                        let value = out.first().unwrap().to_owned();
                        attributes.insert(key, value);
                    }
                }
            }

            let children = el
                .children
                .iter()
                .filter_map(ast_to_xnode)
                .collect::<Vec<XNode>>();

            Some(XNode {
                node_type: NodeType::Element,
                tag: Some(tag),
                text: None,
                attrs: (if attributes.len() > 0 {
                    Some(attributes)
                } else {
                    None
                }),
                children: Some(children),
            })
        }
        Node::Text(text) => Some(XNode {
            node_type: NodeType::Text,
            tag: None,
            text: Some(text.value.value()),
            attrs: None,
            children: None,
        }),
        Node::RawText(text) => Some(XNode {
            node_type: NodeType::Text,
            tag: None,
            text: Some(text.to_string_best()),
            attrs: None,
            children: None,
        }),
        Node::Comment(_)
        | Node::Block(_)
        | Node::Doctype(_)
        | Node::Fragment(_)
        | Node::Custom(_) => None,
    }
}

#[pymodule]
fn xcomponent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NodeType>()?;
    m.add_class::<XNode>()?;
    m.add_function(wrap_pyfunction!(parse_str, m)?)?;
    Ok(())
}
