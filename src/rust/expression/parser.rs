use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use pyo3::exceptions::{PySyntaxError, PyValueError};
use pyo3::PyErr;

use crate::expression::tokens::{ExpressionToken, FunctionCall};
use crate::markup::parser::parse_markup;

use super::tokens::PostfixOp;

#[derive(Parser)]
#[grammar = "rust/expression/grammar.pest"]
pub struct ExpressionParser;

fn parse_expression_tokens(pairs: Pairs<Rule>) -> Vec<ExpressionToken> {
    let mut result = Vec::new();

    for pair in pairs {
        if let Ok(node) = parse_expression_token(pair) {
            if node != ExpressionToken::Noop {
                result.push(node);
            }
        }
    }
    return result;
}

fn parse_expression_token(pair: Pair<Rule>) -> Result<ExpressionToken, String> {
    match pair.as_rule() {
        Rule::expression => parse_expression_token(pair.into_inner().next().unwrap()),
        Rule::field => {
            let inner = pair.into_inner().next().unwrap();
            let postfix = inner.as_str();
            Ok(ExpressionToken::PostfixOp(PostfixOp::Field(
                postfix.to_string(),
            )))
        }
        Rule::index => {
            let mut inner = pair.into_inner();
            let postfix = parse_expression_token(inner.next().unwrap())?;
            Ok(ExpressionToken::PostfixOp(PostfixOp::Index(Box::new(
                postfix,
            ))))
        }
        Rule::binary_expression => {
            let mut inner = pair.into_inner();
            let mut tokens = Vec::new();

            while let Some(p) = inner.next() {
                tokens.push(parse_expression_token(p)?);
            }
            Ok(ExpressionToken::BinaryExpression(tokens))
        }
        Rule::if_expression => {
            let mut inner = pair.into_inner();

            let condition_pair = inner.next().unwrap(); // expression
            let then_pair = inner.next().unwrap(); // block

            let condition = Box::new(parse_expression_token(condition_pair)?);
            let then_branch = Box::new(parse_expression_token(
                then_pair.into_inner().next().unwrap(),
            )?);

            let else_branch = if let Some(else_block) = inner.next() {
                let else_expr = parse_expression_token(else_block.into_inner().next().unwrap())?;
                Some(Box::new(else_expr))
            } else {
                None
            };

            Ok(ExpressionToken::IfExpression {
                condition,
                then_branch,
                else_branch,
            })
        }
        Rule::for_expression => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let iterable_expr = inner.next().unwrap();
            let body_expr = inner.next().unwrap().into_inner().next().unwrap();

            let iterable = Box::new(parse_expression_token(iterable_expr)?);
            let body = Box::new(parse_expression_token(body_expr)?);

            Ok(ExpressionToken::ForExpression {
                ident,
                iterable,
                body,
            })
        }
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let params = parse_expression_tokens(inner);
            debug!("Pushing function call {}({:?})", ident, params);
            Ok(ExpressionToken::FuncCall(FunctionCall::new(ident, params)))
        }
        Rule::ident => {
            let content = pair.as_str();
            debug!("Pushing ident {}", content);
            Ok(ExpressionToken::Ident(content.to_string()))
        }
        Rule::operator => {
            let op = pair.as_str();
            debug!("Pushing operator {}", op);
            Ok(ExpressionToken::Operator(op.parse().unwrap()))
        }
        Rule::integer => {
            let value: isize = pair.as_str().parse().unwrap();
            debug!("Pushing integer {}", value);
            Ok(ExpressionToken::Integer(value))
        }
        Rule::boolean => {
            let value: bool = pair.as_str().parse().unwrap();
            debug!("Pushing boolean {}", value);
            Ok(ExpressionToken::Boolean(value))
        }
        Rule::string => {
            let value = pair.as_str().trim_matches('"');
            debug!("Pushing string {}", value);
            Ok(ExpressionToken::String(value.to_string()))
        }
        Rule::component => {
            debug!("Pushing component");
            let raw = pair.as_str();
            debug!("Pushing component {}", raw);
            parse_markup(raw)
                .map(|n| ExpressionToken::XNode(n))
                .map_err(|e| format!("Syntax error need {}", e))
        }
        _ => {
            warn!("No rule defined for {:?}", pair.as_rule());
            Ok(ExpressionToken::Noop)
        }
    }
}

pub(crate) fn tokenize(raw: &str) -> Result<ExpressionToken, PyErr> {
    let mut pairs = ExpressionParser::parse(Rule::expression, raw.trim())
        .map_err(|e| PySyntaxError::new_err(format!("{}", e)))?;

    if let Some(init) = pairs.next() {
        return parse_expression_token(init).map_err(|e| PySyntaxError::new_err(e));
    }

    Err(PyValueError::new_err(format!(
        "Invalid expression: {} ({:?})",
        raw, pairs
    )))
}
