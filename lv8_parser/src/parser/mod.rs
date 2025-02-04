mod expression;
mod statement;

use pest::{error::LineColLocation, iterators::Pair, Parser};
use std::collections::HashMap;

use crate::Either;
use lv8_common::error::{Error, Result, SyntaxError};

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
pub enum Statement {
    Assignment {
        left: Vec<String>,
        right: Either<Expression, Box<Statement>>,
    },
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: Block,
    },
    FunctionCall {
        expression: Expression,
        arguments: Vec<Either<Expression, Statement>>,
    },
    ModuleDefinition {
        name: String,
        body: Block,
    },
    If {
        condition: Expression,
        body: Block,
        else_if: Vec<(Expression, Block)>,
        else_body: Option<Block>,
    },
    While {
        condition: Expression,
        body: Block,
    },
    Import {
        path: String,
        ident: String,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Null,
    Undefined,
    Boolean(bool),
    Number(Either<isize, f64>),
    String(String),
    Object(HashMap<String, Expression>),
    Array(Vec<Expression>),
    Identifier(String),
    Namespace(Vec<String>),
    MathExpression(MathExpression),
    LogicExpression(LogicExpression),
    ComparisonExpression(ComparisonExpression),
}

#[derive(Clone, Debug)]
pub enum MathExpression {
    Number(Either<isize, f64>),
    Operation {
        left: Box<Expression>,
        operation: MathOperation,
        right: Box<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponentiation,
}

#[derive(Clone, Debug)]
pub enum LogicExpression {
    Not {
        expr: Box<Expression>,
    },
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Boolean(bool),
}

#[derive(Clone, Debug)]
pub struct ComparisonExpression {
    pub left: Box<Expression>,
    pub operation: ComparisonOperation,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug)]
pub enum ComparisonOperation {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

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
            let (line, column) = match e.line_col {
                LineColLocation::Pos((line, column)) => (line, column),
                LineColLocation::Span((line, column), _) => (line, column),
            };

            return Err(Error::syntax(
                e.variant.to_string().as_str(),
                SyntaxError::new(e.line(), line, column),
            ));
        }
    };

    Ok(ASTNode::Block(parse_block(
        pairs.next().unwrap().into_inner().next().unwrap(),
    )?))
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
