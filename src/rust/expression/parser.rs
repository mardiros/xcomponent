use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use crate::expression::tokens::ExpressionToken;
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
        Rule::expression => {
            let inner = pair.into_inner();
            let children = parse_expression_tokens(inner);
            Some(ExpressionToken::Expression(children))
        }
        Rule::ident => {
            debug!("Pushing ident");
            let content = pair.as_str();
            Some(ExpressionToken::Ident(content.to_string()))
        }
        Rule::operator => {
            debug!("Pushing operator");
            let op = pair.as_str();
            Some(ExpressionToken::Operator(op.parse().unwrap()))
        }
        Rule::integer => {
            debug!("Pushing integer");
            let value: usize = pair.as_str().parse().unwrap();
            Some(ExpressionToken::Integer(value))
        }
        Rule::string => {
            debug!("Pushing string");
            let op = pair.as_str().trim_matches('"');
            Some(ExpressionToken::String(op.to_string()))
        }
        Rule::component => {
            debug!("Pushing component");
            let raw = pair.as_str();
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

pub(crate) fn parse_expression(raw: &str) -> Result<Vec<ExpressionToken>, String> {
    let mut pairs = ExpressionParser::parse(Rule::expression, raw).map_err(|x| format!("{}", x))?;

    if let Some(pair) = pairs.next() {
        let tokens = pair
            .into_inner()
            .filter_map(parse_expression_token)
            .collect();

        Ok(tokens)
    } else {
        Ok(Vec::new())
    }
}
