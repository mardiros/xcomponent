use crate::context::Literal;
use crate::expression::tokens::{Operator, UnaryOperator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AST {
    Variable(String),
    Literal(Literal),
    Unary {
        op: UnaryOperator,
        expr: Box<AST>,
    },
    Binary {
        left: Box<AST>,
        op: Operator,
        right: Box<AST>,
    },
    FieldAccess(Box<AST>, String),
    IndexAccess(Box<AST>, Box<AST>),
    CallAccess {
        left: Box<AST>,
        args: Vec<AST>,
        kwargs: HashMap<String, AST>,
    },
    IfStatement {
        condition: Box<AST>,
        then_branch: Box<AST>,
        else_branch: Option<Box<AST>>,
    },
    ForStatement {
        ident: String,
        iterable: Box<AST>,
        body: Box<AST>,
    },
}
