use std::sync::Once;
static INIT: Once = Once::new();

#[macro_use]
extern crate log;

use env_logger;

use pyo3::prelude::*;

mod catalog;
mod context;
mod expression;
mod markup;

use crate::catalog::XCatalog;
use crate::context::RenderContext;
use crate::markup::parser::parse_markup;
use crate::markup::tokens::{NodeType, XComment, XElement, XExpression, XFragment, XNode, XText};

#[pymodule]
fn xcore(m: &Bound<'_, PyModule>) -> PyResult<()> {
    INIT.call_once(|| {
        env_logger::init();
    });

    m.add_class::<NodeType>()?;
    m.add_class::<XNode>()?;
    m.add_class::<XFragment>()?;
    m.add_class::<XElement>()?;
    m.add_class::<XComment>()?;
    m.add_class::<XText>()?;
    m.add_class::<XExpression>()?;
    m.add_class::<XCatalog>()?;
    m.add_class::<RenderContext>()?;

    m.add_function(wrap_pyfunction!(parse_markup, m)?)?;
    Ok(())
}
