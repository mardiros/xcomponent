use std::slice::Iter;

use pyo3::exceptions::PySyntaxError;
use pyo3::prelude::*;

use super::model::{Literal, AST};

use crate::expression::tokens::ExpressionToken;
use crate::expression::tokens::PostfixOp;

pub fn token_to_ast(tok: &ExpressionToken) -> Result<AST, PyErr> {
    let ast = match tok {
        ExpressionToken::BinaryExpression(ex) => Ok(parse(ex.as_slice())?),
        ExpressionToken::String(s) => Ok(AST::Literal(Literal::Str(s.to_string()))),
        // ExpressionToken::Uuid(s) => Ok(AST::Literal(Literal::Uuid(s.to_string()))),
        ExpressionToken::Boolean(b) => Ok(AST::Literal(Literal::Bool(b.clone()))),
        ExpressionToken::Integer(n) => Ok(AST::Literal(Literal::Int(n.clone()))),
        ExpressionToken::Ident(ident) => Ok(AST::Variable(ident.to_string())),
        ExpressionToken::XNode(n) => Ok(AST::Literal(Literal::XNode(n.clone()))),
        ExpressionToken::PostfixOp(op) => {
            // the ast is handled by the
            error!("Should never enter postfix op code : {:?}", op);
            Ok(AST::Literal(Literal::Str("".to_string())))
        }
        ExpressionToken::IfExpression {
            condition,
            then_branch,
            else_branch,
        } => Ok(AST::IfStatement {
            condition: token_to_ast(condition).map(|x| Box::new(x))?,
            then_branch: token_to_ast(then_branch).map(|x| Box::new(x))?,
            else_branch: match else_branch {
                Some(token) => Some(token_to_ast(token).map(|x| Box::new(x))?),
                None => None,
            },
        }),
        ExpressionToken::ForExpression {
            ident,
            iterable,
            body,
        } => Ok(AST::ForStatement {
            ident: ident.clone(),
            iterable: token_to_ast(iterable).map(|x| Box::new(x))?,
            body: token_to_ast(body).map(|x| Box::new(x))?,
        }),
        _ => Err(PySyntaxError::new_err(format!(
            "Syntax error, unexpected token {:?}",
            tok
        ))),
    };
    ast
}

pub fn parse_next(iter: &mut Iter<ExpressionToken>) -> Result<AST, PyErr> {
    let tok = iter
        .next()
        .ok_or(PySyntaxError::new_err("expected at least one token"))?;
    let mut left = token_to_ast(&tok)?;

    while let Some(op_token) = iter.next() {
        error!(">>> {:?}", op_token);
        match op_token {
            ExpressionToken::PostfixOp(op) => match op {
                PostfixOp::Field(f) => left = AST::FieldAccess(Box::new(left), f.clone()),
                PostfixOp::Index(i) => {
                    left = AST::IndexAccess(Box::new(left), Box::new(token_to_ast(&i)?))
                }
                PostfixOp::Call { args, kwargs } => {
                    left = AST::CallAccess {
                        left: Box::new(left),
                        args: args
                            .into_iter()
                            .map(|arg| -> Result<_, _> { token_to_ast(&arg) })
                            .collect::<Result<_, _>>()?,
                        kwargs: kwargs
                            .into_iter()
                            .map(|(k, v)| -> Result<(String, AST), PyErr> {
                                Ok((k.clone(), token_to_ast(&v)?))
                            })
                            .collect::<Result<_, _>>()?,
                    };
                }
            },
            ExpressionToken::Operator(op) => {
                let right = parse_next(iter)?;

                left = AST::Binary {
                    left: Box::new(left),
                    op: op.clone(),
                    right: Box::new(right),
                };
            }
            _ => {
                return Err(PySyntaxError::new_err(format!(
                    "Operator expected, got {:?}",
                    op_token,
                )))
            }
        };
    }

    Ok(left)
}

pub fn parse(tokens: &[ExpressionToken]) -> Result<AST, PyErr> {
    debug!(">>>> Parsing tokens :{:?}", tokens);
    let mut iter = tokens.iter();
    parse_next(&mut iter)
}
