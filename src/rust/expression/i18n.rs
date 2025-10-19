use pyo3::{prelude::*, pyclass, pyfunction, pymethods, Bound, PyResult, Python};

use crate::{
    context::Literal,
    expression::{
        ast::{model::AST, parse::parse},
        parser::tokenize,
    },
    markup::tokens::XNode,
};

#[pyclass]
#[derive(Debug, Clone)]
pub enum MessageKind {
    Gettext {
        message: String,
    },
    Dgettext {
        domain: String,
        message: String,
    },
    Ngettext {
        singular: String,
        plural: String,
    },
    Dngettext {
        domain: String,
        singular: String,
        plural: String,
    },
    Pgettext {
        context: String,
        message: String,
    },
    Dpgettext {
        domain: String,
        context: String,
        message: String,
    },
    Npgettext {
        context: String,
        singular: String,
        plural: String,
    },
    Dnpgettext {
        domain: String,
        context: String,
        singular: String,
        plural: String,
    },
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ExtractedMessage {
    lineno: usize,
    funcname: String,
    message: MessageKind,
    comments: Vec<String>,
}

impl ExtractedMessage {
    fn new(lineno: usize, funcname: String, message: MessageKind, comments: Vec<String>) -> Self {
        ExtractedMessage {
            lineno,
            funcname,
            message,
            comments,
        }
    }
}

#[pymethods]
impl ExtractedMessage {
    #[getter]
    pub fn lineno<'py>(&self) -> usize {
        self.lineno
    }

    #[getter]
    pub fn funcname<'py>(&self) -> &str {
        self.funcname.as_str()
    }

    #[getter]
    pub fn message<'py>(&self, py: Python<'py>) -> Bound<'py, PyAny> {
        let c = self.message.clone().into_pyobject(py);
        c.unwrap().into_any()
    }

    #[getter]
    pub fn comments<'py>(&self, py: Python<'py>) -> Bound<'py, PyAny> {
        self.comments.clone().into_pyobject(py).unwrap()
    }
}

fn extract_from_ast(ast: AST) -> PyResult<Vec<ExtractedMessage>> {
    let mut res = Vec::new();
    match ast {
        AST::CallAccess {
            left,
            args,
            kwargs: _,
        } => match *left {
            AST::FieldAccess(_, s) => match s.as_str() {
                "gettext" => match args.first() {
                    Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                        0,
                        s.clone(),
                        MessageKind::Gettext { message: v.clone() },
                        Vec::new(),
                    )),
                    _ => (),
                },
                "ngettext" => {
                    let singular = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Ngettext {
                                singular,
                                plural: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "dgettext" => {
                    let domain = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Dgettext {
                                domain,
                                message: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "dngettext" => {
                    let domain = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    let singular = match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(2) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Dngettext {
                                domain,
                                singular,
                                plural: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "pgettext" => {
                    let context = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Pgettext {
                                context,
                                message: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "dpgettext" => {
                    let domain = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    let context = match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(2) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Dpgettext {
                                domain,
                                context,
                                message: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "npgettext" => {
                    let context = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    let singular = match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(2) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Npgettext {
                                context,
                                singular,
                                plural: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                "dnpgettext" => {
                    let domain = match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    let context = match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    let singular = match args.get(2) {
                        Some(AST::Literal(Literal::Str(v))) => v.clone(),
                        _ => "".to_owned(),
                    };
                    match args.get(3) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(ExtractedMessage::new(
                            0,
                            s.clone(),
                            MessageKind::Dnpgettext {
                                domain,
                                context,
                                singular,
                                plural: v.clone(),
                            },
                            Vec::new(),
                        )),
                        _ => (),
                    }
                }
                _ => {}
            },
            _ => {}
        },
        AST::IfStatement {
            condition: _,
            then_branch,
            else_branch,
        } => {
            res.extend(extract_from_ast(*then_branch.clone())?);
            if else_branch.is_some() {
                res.extend(extract_from_ast(*else_branch.unwrap().clone())?);
            }
        }
        AST::ForStatement {
            ident: _,
            iterable: _,
            body,
        } => {
            res.extend(extract_from_ast(*body.clone())?);
        }
        AST::LetStatement { ident: _, expr } => {
            res.extend(extract_from_ast(*expr.clone())?);
        }
        AST::Literal(Literal::XNode(XNode::Element(node))) => {
            for child in node.attrs().values() {
                if let XNode::Expression(expr) = child {
                    res.extend(extract_expr_i18n_messages(expr.expression())?);
                }
            }
            for child in node.children() {
                if let XNode::Expression(expr) = child {
                    res.extend(extract_expr_i18n_messages(expr.expression())?);
                }
            }
        }
        AST::Literal(Literal::XNode(XNode::Fragment(node))) => {
            for child in node.children() {
                match child {
                    XNode::Expression(expr) => {
                        res.extend(extract_expr_i18n_messages(expr.expression())?);
                    }
                    XNode::Element(node) => {
                        for child in node.attrs().values() {
                            if let XNode::Expression(expr) = child {
                                res.extend(extract_expr_i18n_messages(expr.expression())?);
                            }
                        }
                        for child in node.children() {
                            if let XNode::Expression(expr) = child {
                                res.extend(extract_expr_i18n_messages(expr.expression())?);
                            }
                        }
                    }
                    _ => {
                        warn!("Ignoring {:?} while extracting messages", child);
                    }
                }
            }
        }
        AST::Literal(Literal::XNode(XNode::Expression(expr))) => {
            res.extend(extract_expr_i18n_messages(expr.expression())?);
        }
        _ => {
            warn!("Ignoring {:?} while extracting messages", ast);
        }
    }
    Ok(res)
}

#[pyfunction]
pub(crate) fn extract_expr_i18n_messages(raw: &str) -> PyResult<Vec<ExtractedMessage>> {
    let token = tokenize(raw)?;
    let ast = parse(&[token], 0)?;

    extract_from_ast(ast)
}
