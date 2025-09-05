use std::cmp::min;
use std::collections::HashMap;

use pyo3::prelude::*;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use crate::markup::tokens::{
    XComment, XDocType, XElement, XExpression, XFragment, XNode, XScriptElement, XText,
};

#[derive(Parser)]
#[grammar = "rust/markup/grammar.pest"]
pub struct XParser;

fn parse_nodes(pairs: Pairs<Rule>) -> Vec<XNode> {
    let mut result = Vec::new();

    for pair in pairs {
        if let Some(node) = parse_node(pair) {
            result.push(node);
        }
    }
    return result;
}

fn parse_node(pair: Pair<Rule>) -> Option<XNode> {
    match pair.as_rule() {
        Rule::normal_element => {
            debug!("Pushing normal_element");
            let mut inner = pair.into_inner();
            let open_tag = inner.next().unwrap();
            let (name, attrs) = parse_open_tag(open_tag);

            match name.as_str() {
                "script" | "style" => {
                    let body = inner.as_str();
                    Some(XNode::ScriptElement(XScriptElement::new(
                        name,
                        attrs,
                        body.to_string(),
                    )))
                }
                _ => {
                    let mut children = parse_nodes(inner);
                    // we make the distinctions between self closing element
                    // and normal element from the user input, we must ensure that
                    // the normal element are still rendered as normal element since
                    // it is a user choice.
                    if children.len() == 0 {
                        children.push(XNode::Text(XText::new("".to_string())));
                    }

                    Some(XNode::Element(XElement::new(name, attrs, children)))
                }
            }
        }
        Rule::fragment => {
            debug!("Pushing fragment");
            let inner = pair.into_inner();
            let children = parse_nodes(inner);
            Some(XNode::Fragment(XFragment::new(children)))
        }
        Rule::self_closing_element => {
            debug!("Pushing self_closing_element");
            let (name, attrs) = parse_open_tag(pair);

            Some(XNode::Element(XElement::new(name, attrs, Vec::new())))
        }
        Rule::doctype => {
            debug!("Pushing doctype");
            let content = pair.as_str();
            Some(XNode::DocType(XDocType::new(content.to_string())))
        }
        Rule::comment => {
            debug!("Pushing comment");
            let content = pair.as_str();
            let trimmed = content.trim_start_matches("<!--").trim_end_matches("-->");
            Some(XNode::Comment(XComment::new(trimmed.to_string())))
        }
        Rule::expression => {
            debug!("Pushing expression");
            let content = pair.as_str();
            Some(XNode::Expression(XExpression::new(
                content[1..content.len() - 1].to_string(),
            )))
        }
        Rule::text => {
            debug!("Pushing text");
            let text = pair.as_str();
            if text.trim().len() > 0 {
                Some(XNode::Text(XText::new(text.to_string())))
            } else {
                None
            }
        }
        _ => {
            debug!("No rule defined for {:?}", pair.as_rule());
            None
        }
    }
}

fn parse_open_tag(pair: Pair<Rule>) -> (String, HashMap<String, XNode>) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    let mut attrs = HashMap::new();
    for attr in inner {
        if attr.as_rule() == Rule::attribute {
            let mut parts = attr.into_inner();
            let key = parts.next().unwrap().as_str().to_string();
            if let Some(value_pair) = parts.next() {
                let value = value_pair.as_str();
                if value.starts_with('{') {
                    attrs.insert(
                        key,
                        XNode::Expression(XExpression::new(value[1..value.len() - 1].to_string())),
                    );
                } else {
                    attrs.insert(
                        key,
                        XNode::Text(XText::new(value[1..value.len() - 1].to_string())),
                    );
                }
            } else {
                attrs.insert(key, XNode::Expression(XExpression::new("true".to_string())));
            }
        }
    }

    (name, attrs)
}

#[pyfunction]
pub fn parse_markup(raw: &str) -> PyResult<XNode> {
    let raw = raw.trim();
    info!("Parsing markup {}...", &raw[..min(raw.len(), 24)]);
    debug!("{}", raw);
    let mut pairs = XParser::parse(Rule::document, raw).map_err(|e| {
        return pyo3::exceptions::PyValueError::new_err(format!("Invalid Markup: {}", e));
    })?;

    let pair = pairs
        .next()
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Expected one node"))?;

    if let Some(token) = parse_node(pair) {
        debug!("Token parsed {:?}", token);
        Ok(token)
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(
            "Expected one node, use <></> to represent an empty node",
        ))
    }
}
