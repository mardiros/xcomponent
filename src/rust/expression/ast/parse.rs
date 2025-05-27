use std::slice::Iter;

use pyo3::exceptions::PySyntaxError;
use pyo3::prelude::*;

use crate::context::Literal;
use crate::expression::{
    ast::model::AST,
    tokens::{ExpressionToken, PostfixOp},
};

pub fn token_to_ast(tok: &ExpressionToken, min_prec: u8) -> Result<AST, PyErr> {
    let ast = match tok {
        ExpressionToken::UnaryExpression { op, expr } => {
            let inner = token_to_ast(expr, min_prec)?;
            Ok(AST::Unary {
                op: op.clone(),
                expr: Box::new(inner),
            })
        }
        ExpressionToken::BinaryExpression(ex) => {
            let mut exp = ex.clone();
            Ok(parse(&mut exp, min_prec)?)
        }
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
            condition: token_to_ast(condition, min_prec).map(|x| Box::new(x))?,
            then_branch: token_to_ast(then_branch, min_prec).map(|x| Box::new(x))?,
            else_branch: match else_branch {
                Some(token) => Some(token_to_ast(token, min_prec).map(|x| Box::new(x))?),
                None => None,
            },
        }),
        ExpressionToken::ForExpression {
            ident,
            iterable,
            body,
        } => Ok(AST::ForStatement {
            ident: ident.clone(),
            iterable: token_to_ast(iterable, min_prec).map(|x| Box::new(x))?,
            body: token_to_ast(body, min_prec).map(|x| Box::new(x))?,
        }),
        // Comment produce a Noop
        ExpressionToken::Noop => Ok(AST::Literal(Literal::Str("".to_string()))),
        _ => Err(PySyntaxError::new_err(format!(
            "Syntax error, unexpected token {:?}",
            tok
        ))),
    };
    ast
}

pub fn get_left(iter: &mut std::iter::Peekable<Iter<ExpressionToken>>) -> Result<AST, PyErr> {
    loop {
        let tok = iter
            .next()
            .ok_or(PySyntaxError::new_err("expected at least one token"))?;
        match tok {
            ExpressionToken::Noop => {}
            _ => {
                return token_to_ast(&tok, 0);
            }
        }
    }
}

fn get_next_token(
    iter: &mut std::iter::Peekable<Iter<ExpressionToken>>,
    min_prec: u8,
) -> Result<AST, PyErr> {
    let mut left = get_left(iter)?;

    while let Some(token) = iter.peek() {
        match token {
            ExpressionToken::Operator(op) if op.precedence() >= min_prec => {
                let op = if let Some(ExpressionToken::Operator(op)) = iter.next() {
                    op.clone()
                } else {
                    break;
                };

                let right = get_next_token(iter, op.precedence() + 1)?;
                left = AST::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            ExpressionToken::PostfixOp(_) => {
                let op = iter.next().unwrap(); // safe to consume
                match op {
                    ExpressionToken::PostfixOp(PostfixOp::Field(f)) => {
                        left = AST::FieldAccess(Box::new(left), f.clone())
                    }
                    ExpressionToken::PostfixOp(PostfixOp::Index(i)) => {
                        left =
                            AST::IndexAccess(Box::new(left), Box::new(token_to_ast(&i, min_prec)?))
                    }
                    ExpressionToken::PostfixOp(PostfixOp::Call { args, kwargs }) => {
                        left = AST::CallAccess {
                            left: Box::new(left),
                            args: args
                                .into_iter()
                                .map(|arg| -> Result<_, _> { token_to_ast(&arg, min_prec) })
                                .collect::<Result<_, _>>()?,
                            kwargs: kwargs
                                .into_iter()
                                .map(|(k, v)| -> Result<(String, AST), PyErr> {
                                    Ok((k.clone(), token_to_ast(&v, min_prec)?))
                                })
                                .collect::<Result<_, _>>()?,
                        };
                    }
                    _ => unreachable!(),
                }
            }
            ExpressionToken::Noop => {
                iter.next(); // consume and ignore
            }
            _ => break,
        }
    }

    Ok(left)
}

pub fn parse(tokens: &[ExpressionToken], min_prec: u8) -> Result<AST, PyErr> {
    debug!(">>>> Parsing tokens :{:?}", tokens);
    let mut iter = tokens.iter().peekable();
    get_next_token(&mut iter, min_prec)
}
