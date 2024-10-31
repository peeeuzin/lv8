use pest::iterators::Pair;

use crate::Either;

use super::{Rule, Statement};

use crate::error::Result;

pub fn parse(pair: Pair<Rule>) -> Result<Statement> {
    match pair.as_rule() {
        Rule::assign => {
            let mut inner = pair.into_inner();

            let var_list = inner.next().unwrap();
            let expr = inner.next().unwrap();

            let mut left: Vec<String> = vec![];

            for var in var_list.into_inner() {
                left.push(var.as_str().to_string());
            }

            let right = match expr.as_rule() {
                Rule::expr => {
                    Either::Left(super::expression::parse(expr.into_inner().next().unwrap())?)
                }
                Rule::stmt => Either::Right(Box::new(parse(expr.into_inner().next().unwrap())?)),
                _ => unreachable!("unreachable!() in statement.rs, {:?}", expr.as_rule()),
            };

            Ok(Statement::Assignment { left, right })
        }

        Rule::function_call => {
            let mut inner = pair.into_inner();

            let name = inner.next().unwrap();
            let name = name.as_str().to_string();

            let mut args = Vec::new();

            for pair in inner {
                match pair.as_rule() {
                    Rule::expr => args.push(Either::Left(super::expression::parse(
                        pair.into_inner().next().unwrap(),
                    )?)),
                    Rule::stmt => {
                        args.push(Either::Right(parse(pair.into_inner().next().unwrap())?))
                    }
                    _ => unreachable!("unreachable!() in statement.rs, {:?}", pair.as_rule()),
                }
            }

            Ok(Statement::FunctionCall {
                name,
                arguments: args,
            })
        }

        Rule::function_def => {
            let mut pairs = pair.into_inner();

            let name = pairs.next().unwrap().as_str().to_string();

            let mut params = vec![];

            let body_or_params = pairs.next().unwrap();

            let body = match body_or_params.as_rule() {
                Rule::name_list => {
                    params = body_or_params
                        .into_inner()
                        .map(|x| x.as_str().to_string())
                        .collect::<Vec<String>>();

                    super::parse_block(pairs.next().unwrap())?
                }
                Rule::block => super::parse_block(body_or_params)?,
                _ => unreachable!(
                    "unreachable!() in statement.rs, {:?}",
                    body_or_params.as_rule()
                ),
            };

            Ok(Statement::FunctionDefinition {
                name,
                parameters: params,
                body,
            })
        }

        Rule::if_statement => {
            let mut pairs = pair.into_inner();

            let condition =
                super::expression::parse(pairs.next().unwrap().into_inner().next().unwrap())?;

            let body = super::parse_block(pairs.next().unwrap())?;

            let mut else_if = Vec::new();
            let mut else_body = None;

            while let Some(pair) = pairs.next() {
                match pair.as_rule() {
                    // else if statement
                    Rule::logic_expr => {
                        let condition = super::expression::parse(pair.clone())?;
                        let body = super::parse_block(pairs.next().unwrap())?;

                        else_if.push((condition, body));
                    }

                    // else statement
                    Rule::block => {
                        let body = super::parse_block(pair)?;

                        else_body = Some(body);
                    }

                    _ => unreachable!("unreachable!() in statement.rs, {:?}", pair.as_rule()),
                }
            }

            Ok(Statement::If {
                condition,
                body,
                else_if,
                else_body,
            })
        }

        Rule::while_statement => {
            let mut pairs = pair.into_inner();

            let condition =
                super::expression::parse(pairs.next().unwrap().into_inner().next().unwrap())?;

            let body = super::parse_block(pairs.next().unwrap())?;

            Ok(Statement::While { condition, body })
        }

        _ => unreachable!("unreachable!() in statement.rs, {:?}", pair.as_rule()),
    }
}
