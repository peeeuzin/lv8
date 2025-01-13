use pest::{iterators::Pair, pratt_parser::PrattParser};

use super::{
    ComparisonExpression, ComparisonOperation, Expression, LogicExpression, MathExpression,
    MathOperation, Rule,
};
use crate::Either;
use lv8_common::error::Result;

lazy_static::lazy_static! {
    static ref MATH_PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use super::super::Rule;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
            .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left) | Op::infix(Rule::modulus, Left))
            .op(Op::infix(Rule::exponentiation, Right))
    };
}

lazy_static::lazy_static! {
    static ref LOGIC_PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use super::super::Rule;

        // Precedence is defined lowest to highest
        PrattParser::new()
        .op(Op::infix(Rule::and, Left) | Op::infix(Rule::or, Left))
        .op(Op::prefix(Rule::not))
        .op(
            Op::infix(Rule::greather_eq, Left)
            | Op::infix(Rule::less_eq, Left)
            | Op::infix(Rule::greather, Left)
            | Op::infix(Rule::less, Left)
            | Op::infix(Rule::equal, Left)
            | Op::infix(Rule::not_equal, Left)
        )
    };
}

pub fn parse(pair: Pair<Rule>) -> Result<Expression> {
    match pair.as_rule() {
        Rule::null => Ok(Expression::Null),

        Rule::undefined => Ok(Expression::Undefined),

        Rule::boolean => Ok(Expression::Boolean(pair.as_str().parse().unwrap())),

        Rule::number => {
            let inner = pair.into_inner().next().unwrap();

            match inner.as_rule() {
                Rule::integer => Ok(Expression::Number(Either::Left(
                    inner.as_str().parse().unwrap(),
                ))),
                Rule::float => Ok(Expression::Number(Either::Right(
                    inner.as_str().parse().unwrap(),
                ))),
                _ => unreachable!(),
            }
        }
        Rule::ident => Ok(Expression::Identifier(pair.as_str().to_string())),

        Rule::string => {
            let inner = pair.into_inner().next().unwrap();
            Ok(Expression::String(inner.as_str().to_string()))
        }

        Rule::object => {
            let mut object = std::collections::HashMap::new();

            for pair in pair.into_inner() {
                let mut inner = pair.into_inner();

                let key = match parse(inner.next().unwrap())? {
                    Expression::String(key) => key,
                    Expression::Identifier(key) => key,
                    _ => unreachable!(),
                };

                let value = parse(inner.next().unwrap())?;
                object.insert(key, value);
            }

            Ok(Expression::Object(object))
        }

        Rule::array => {
            let mut array = Vec::new();

            for pair in pair.into_inner() {
                array.push(parse(pair)?);
            }

            Ok(Expression::Array(array))
        }

        Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            Ok(parse(inner)?)
        }

        Rule::math_expr => parse_math_expression(pair),

        Rule::logic_expr => parse_logic_expression(pair),

        Rule::namespace => {
            let mut namespace = Vec::new();

            for pair in pair.into_inner() {
                namespace.push(pair.as_str().to_string());
            }

            Ok(Expression::Namespace(namespace))
        }

        _ => unreachable!("unreachable!() in expression.rs, {:?}", pair.as_rule()),
    }
}

fn parse_math_expression(pair: Pair<Rule>) -> Result<Expression> {
    let pairs = pair.into_inner();

    MATH_PRATT_PARSER
        .map_primary(parse)
        .map_infix(|left, op, right| {
            let op = match op.as_rule() {
                Rule::add => MathOperation::Add,
                Rule::subtract => MathOperation::Subtract,
                Rule::multiply => MathOperation::Multiply,
                Rule::divide => MathOperation::Divide,
                Rule::exponentiation => MathOperation::Exponentiation,
                Rule::modulus => MathOperation::Modulus,
                _ => unreachable!("unreachable!() in expression.rs, {:?}", op.as_rule()),
            };

            let math_expr = MathExpression::Operation {
                left: Box::new(left?),
                operation: op,
                right: Box::new(right?),
            };

            Ok(Expression::MathExpression(math_expr))
        })
        .parse(pairs)
}

fn parse_logic_expression(pair: Pair<Rule>) -> Result<Expression> {
    let pairs = pair.into_inner();

    LOGIC_PRATT_PARSER
        .map_primary(parse)
        .map_prefix(|_, right| {
            let right = right?;

            let logic_expr = LogicExpression::Not {
                expr: Box::new(right),
            };

            Ok(Expression::LogicExpression(logic_expr))
        })
        .map_infix(|left, op, right| match op.as_rule() {
            Rule::and => Ok(Expression::LogicExpression(LogicExpression::And {
                left: Box::new(left?),
                right: Box::new(right?),
            })),
            Rule::or => Ok(Expression::LogicExpression(LogicExpression::Or {
                left: Box::new(left?),
                right: Box::new(right?),
            })),
            _ => {
                let op = match op.as_rule() {
                    Rule::equal => ComparisonOperation::Equal,
                    Rule::not_equal => ComparisonOperation::NotEqual,
                    Rule::greather => ComparisonOperation::GreaterThan,
                    Rule::less => ComparisonOperation::LessThan,
                    Rule::greather_eq => ComparisonOperation::GreaterThanOrEqual,
                    Rule::less_eq => ComparisonOperation::LessThanOrEqual,
                    _ => unreachable!("unreachable!() in expression.rs, {:?}", op.as_rule()),
                };

                let comp_expr = ComparisonExpression {
                    left: Box::new(left?),
                    operation: op,
                    right: Box::new(right?),
                };

                Ok(Expression::ComparisonExpression(comp_expr))
            }
        })
        .parse(pairs)
}
