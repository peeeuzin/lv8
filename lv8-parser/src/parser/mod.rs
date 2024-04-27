mod error;
mod expression;
mod statement;

use std::collections::HashMap;

use error::{GrammarError, Result};
use pest::{iterators::Pair, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/lv8.pest"]
struct LV8Parser;

#[derive(Debug)]
pub enum ASTNode {
    Block(Block),
}

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Statement>, pub ReturnStatement);

#[derive(Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment {
        left: Vec<Namespace>,
        right: Either<Expression, Box<Statement>>,
    },
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: Block,
    },
    FunctionCall {
        name: Namespace,
        arguments: Vec<Either<Expression, Statement>>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Null,
    Undefined,
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(String),
    Object(HashMap<String, Expression>),
    Array(Vec<Expression>),
    Identifier(String),
}

#[derive(Clone, Debug)]
pub struct Namespace(pub Vec<String>);

#[derive(Clone, Debug)]
pub enum ReturnStatement {
    Return(Expression),
    Break,
    Continue,
}

pub fn parse(input: &str) -> Result<ASTNode> {
    let mut pairs = match LV8Parser::parse(Rule::program, input) {
        Ok(e) => e,
        Err(e) => {
            return Err(GrammarError::with_message(&e.to_string()));
        }
    };

    Ok(ASTNode::Block(parse_block(
        pairs.next().unwrap().into_inner().next().unwrap(),
    )?))
}

fn parse_namespace(pair: Pair<Rule>) -> Namespace {
    let mut namespace = Vec::new();

    match pair.as_rule() {
        Rule::namespace => {
            let pair = pair.into_inner();

            for pair in pair {
                namespace.push(pair.as_str().to_string());
            }
        }
        Rule::ident => namespace.push(pair.as_str().to_string()),

        _ => unreachable!("Unknown rule: {:?}", pair.as_rule()),
    }

    Namespace(namespace)
}

fn parse_block(pair: Pair<Rule>) -> Result<Block> {
    match pair.as_rule() {
        Rule::block => {
            let pair = pair.into_inner();

            let mut statements = Vec::new();
            let mut return_statement = ReturnStatement::Return(Expression::Undefined);

            for pair in pair {
                match pair.as_rule() {
                    Rule::stmt => statements
                        .push(statement::parse(pair.into_inner().next().unwrap()).unwrap()),
                    Rule::return_statement => {
                        let expr = pair.into_inner().next().unwrap();

                        match expr.as_rule() {
                            Rule::expr => {
                                return_statement = ReturnStatement::Return(expression::parse(expr)?)
                            }

                            _ => {
                                return_statement = match expr.as_str() {
                                    "break" => ReturnStatement::Break,
                                    "continue" => ReturnStatement::Continue,
                                    _ => unreachable!("Unknown rule: {:?}", expr.as_rule()),
                                }
                            }
                        }
                    }
                    _ => unreachable!("Unknown rule: {:?}", pair.as_rule()),
                }
            }

            Ok(Block(statements, return_statement))
        }
        _ => unreachable!("Unknown rule: {:?}", pair.as_rule()),
    }
}
