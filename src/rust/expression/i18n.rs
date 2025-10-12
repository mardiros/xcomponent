use pyo3::{pyfunction, PyResult};

use crate::{
    context::Literal,
    expression::{
        ast::{model::AST, parse::parse},
        parser::tokenize,
    },
};

fn extract_from_ast(ast: AST) -> Vec<String> {
    let mut res = Vec::new();
    match ast {
        AST::CallAccess {
            left,
            args,
            kwargs: _,
        } => match *left {
            AST::FieldAccess(_, s) => {
                if s == "gettext" {
                    match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => res.push(v.clone()),
                        _ => (),
                    }
                } else if s == "ngettext" {
                    match args.first() {
                        Some(AST::Literal(Literal::Str(v))) => res.push(v.clone()),
                        _ => (),
                    }
                    match args.get(1) {
                        Some(AST::Literal(Literal::Str(v))) => res.push(v.clone()),
                        _ => (),
                    }
                }
            }
            _ => {}
        },
        _ => {
            error!("??? {:?}", ast);
        }
    }
    res
}

#[pyfunction]
pub(crate) fn extract_expr_i18n_messages(raw: &str) -> PyResult<Vec<String>> {
    let token = tokenize(raw)?;
    let ast = parse(&[token], 0)?;

    Ok(extract_from_ast(ast))
}
