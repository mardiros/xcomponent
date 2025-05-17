use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use pyo3::exceptions::{PySyntaxError, PyValueError};
use pyo3::PyErr;

use crate::expression::tokens::{ExpressionToken, FunctionCall};
use crate::markup::parser::parse_markup;

#[derive(Parser)]
#[grammar = "rust/expression/grammar.pest"]
pub struct ExpressionParser;

fn parse_expression_tokens(pairs: Pairs<Rule>) -> Vec<ExpressionToken> {
    let mut result = Vec::new();

    for pair in pairs {
        if let Some(node) = parse_expression_token(pair) {
            result.push(node);
        }
    }
    return result;
}

fn parse_expression_token(pair: Pair<Rule>) -> Option<ExpressionToken> {
    match pair.as_rule() {
        Rule::expression => parse_expression_token(pair.into_inner().next()?),
        Rule::field_access => {
            let mut inner = pair.into_inner();
            let base = parse_expression_token(inner.next()?)?;

            inner.fold(Some(base), |acc, segment| {
                let base_expr = acc?;
                match segment.as_rule() {
                    Rule::ident => Some(ExpressionToken::FieldAccess(
                        Box::new(base_expr),
                        segment.as_str().to_string(),
                    )),
                    _ => None,
                }
            })
        }
        Rule::binary_expression => {
            let inner = pair.into_inner();
            let children = parse_expression_tokens(inner);
            Some(ExpressionToken::BinaryExpression(children))
        }
        Rule::if_expression => {
            let mut inner = pair.into_inner();

            let condition_pair = inner.next()?; // expression
            let then_pair = inner.next()?; // block

            let condition = Box::new(parse_expression_token(condition_pair)?);
            let then_branch = Box::new(parse_expression_token(then_pair.into_inner().next()?)?);

            let else_branch = if let Some(else_block) = inner.next() {
                let else_expr = parse_expression_token(else_block.into_inner().next()?)?;
                Some(Box::new(else_expr))
            } else {
                None
            };

            Some(ExpressionToken::IfExpression {
                condition,
                then_branch,
                else_branch,
            })
        }
        Rule::for_expression => {
            let mut inner = pair.into_inner();
            let ident = inner.next()?.as_str().to_string();
            let iterable_expr = inner.next()?;
            let body_expr = inner.next()?.into_inner().next()?;

            let iterable = Box::new(parse_expression_token(iterable_expr)?);
            let body = Box::new(parse_expression_token(body_expr)?);

            Some(ExpressionToken::ForExpression {
                ident,
                iterable,
                body,
            })
        }
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let ident = inner.next()?.as_str().to_string();
            let params = parse_expression_tokens(inner);
            debug!("Pushing function call {}({:?})", ident, params);
            Some(ExpressionToken::FuncCall(FunctionCall::new(ident, params)))
        }
        Rule::ident => {
            let content = pair.as_str();
            debug!("Pushing ident {}", content);
            Some(ExpressionToken::Ident(content.to_string()))
        }
        Rule::operator => {
            let op = pair.as_str();
            debug!("Pushing operator {}", op);
            Some(ExpressionToken::Operator(op.parse().unwrap()))
        }
        Rule::integer => {
            let value: isize = pair.as_str().parse().unwrap();
            debug!("Pushing integer {}", value);
            Some(ExpressionToken::Integer(value))
        }
        Rule::boolean => {
            let value: bool = pair.as_str().parse().unwrap();
            debug!("Pushing boolean {}", value);
            Some(ExpressionToken::Boolean(value))
        }
        Rule::string => {
            let value = pair.as_str().trim_matches('"');
            debug!("Pushing string {}", value);
            Some(ExpressionToken::String(value.to_string()))
        }
        Rule::component => {
            debug!("Pushing component");
            let raw = pair.as_str();
            debug!("Pushing component {}", raw);
            let res = parse_markup(raw);
            if let Ok(n) = res {
                Some(ExpressionToken::XNode(n))
            } else {
                error!("FIXME, raise python error {}", res.unwrap_err());
                None
            }
        }
        _ => {
            warn!("No rule defined for {:?}", pair.as_rule());
            None
        }
    }
}

pub(crate) fn parse_expression(raw: &str) -> Result<ExpressionToken, PyErr> {
    let mut pairs = ExpressionParser::parse(Rule::expression, raw.trim())
        .map_err(|e| PySyntaxError::new_err(format!("{}", e)))?;

    if let Some(init) = pairs.next() {
        if let Some(exp) = parse_expression_token(init) {
            return Ok(exp);
        }
    }

    Err(PyValueError::new_err(format!(
        "Invalid expression: {} ({:?})",
        raw, pairs
    )))

    // if let Some(pair) = pairs.next().unwrap().into_inner().next() {
    //     let tokens = if pair.as_rule() == Rule::binary_expression {
    //         let inner = pair.into_inner();
    //         parse_expression_tokens(inner)
    //     } else {
    //         error!(">>>>>>>>>>>>>>>>>>");
    //     };

    //     Ok(tokens)
    // } else {
    //     Err(PyValueError::new_err(format!(
    //         "Invalid expression: {}",
    //         raw
    //     )))
    // }
}
